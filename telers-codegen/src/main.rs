use clap::Parser;
use std::{
    collections::HashSet,
    fmt::Display,
    fs,
    path::{Path, PathBuf},
    process,
};
use telers_codegen::{
    file::{camel_to_filename, check_tokens_to_file, write_tokens_to_file},
    generator::{self, methods::struct_name_to_method_name},
    parser::api::Schema,
};

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    /// Path to generated directory
    #[arg(
        long,
        visible_alias = "gen-dir",
        value_name = "PATH",
        default_value = "./generated"
    )]
    generated_dir_path: PathBuf,

    /// Path to schema JSON file
    #[arg(long, visible_alias = "schema", value_name = "PATH")]
    schema_json_path: PathBuf,

    /// Generate serde tests in a single integration test file
    #[arg(
        long,
        visible_alias = "tests",
        default_value_t = false,
        requires = "tests_types_path"
    )]
    generate_tests: bool,

    /// Rust path used in generated tests import: `use <path>::types::*;`
    #[arg(long, visible_alias = "types-path")]
    tests_types_path: Option<String>,

    /// Check that the committed output matches the generator instead of writing it
    #[arg(long, default_value_t = false)]
    check: bool,
}

/// Where generated files go, and in `--check` mode where the drift is collected.
struct Output {
    check: bool,
    drifted: Vec<String>,
}

impl Output {
    fn new(check: bool) -> Self {
        Self {
            check,
            drifted: Vec::new(),
        }
    }

    fn write(&mut self, tokens: &impl Display, dir: &Path, filename: &str) {
        let result = if self.check {
            check_tokens_to_file(tokens, dir, filename)
        } else {
            write_tokens_to_file(tokens, dir, filename).map(|()| None)
        };

        match result {
            Ok(None) => {}
            Ok(Some(path)) => self.drifted.push(path),
            Err(err) => {
                eprintln!(
                    "Failed to render file '{filename}' in dir '{dir}': {err}\nContent: {tokens}",
                    dir = dir.display()
                );
                process::exit(1);
            }
        }
    }

    /// Report the drifted files and fail, so a CI job can gate on this.
    fn finish(&self) {
        if !self.check || self.drifted.is_empty() {
            return;
        }

        eprintln!("{} file(s) differ from the generator:", self.drifted.len());
        for path in &self.drifted {
            eprintln!("  {path}");
        }
        eprintln!("Run `just codegen` and commit the result.");
        process::exit(1);
    }
}

fn apply_schema_transforms(schema: &mut telers_codegen::parser::api::NormalizedSchema) {
    schema.skip_types(&["InputFile"]);
    schema.split_message_types();
    schema.split_external_reply_info_types();
    schema.split_update_types();
    schema.split_chat_types();
    schema.split_sticker_types();
    schema.split_poll_types();
    schema.split_poll_media_types();
    schema.split_giveaway_types();
    schema.split_giveaway_winners_types();
    schema.split_star_transaction_types();
    schema.split_encrypted_passport_element_types();
    schema.split_message_entity_types();
    schema.split_inline_query_result();
    schema.split_transaction_partner_user_types();
    schema.compose_reply_markup_type();
    schema.compose_input_rich_message_media_type();
    schema.reorder_untagged_subtypes();
    schema.add_unknown_fallbacks();
    schema.modify_get_updates_returns_method();
}

fn generate_enums(
    schema: &telers_codegen::parser::api::NormalizedSchema,
    src_dir: &Path,
    output: &mut Output,
) {
    let enums_dir = src_dir.join("enums");
    let enum_names = schema
        .types
        .iter()
        .filter(|(_, ty)| !ty.subtypes.is_empty())
        .filter_map(|(name, ty)| {
            let tokens = generator::enums::tokenize_kind_enum_file(ty)?;
            output.write(&tokens, &enums_dir, &camel_to_filename(name, Some("rs")));
            Some(name.as_str())
        })
        .collect::<Vec<_>>();
    let mut own_enum_names = vec![];
    for (name, tokens) in &generator::enums::tokenize_own_enums(schema) {
        output.write(tokens, &enums_dir, &camel_to_filename(name, Some("rs")));
        own_enum_names.push(*name);
    }
    let tokens =
        generator::enums::tokenize_kind_enums_mod(enum_names.as_slice(), own_enum_names.as_slice());
    output.write(&tokens, src_dir, "enums.rs");
}

fn main() {
    let args = Args::parse();
    let mut output = Output::new(args.check);

    let schema_content = fs::read_to_string(&args.schema_json_path).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read schema file '{}': {err}",
            args.schema_json_path.display()
        );
        process::exit(1);
    });

    let mut schema = Schema::parse_from_json(&schema_content)
        .unwrap_or_else(|err| {
            eprintln!("Failed to parse schema file: {err}");
            process::exit(1);
        })
        .normalize();

    apply_schema_transforms(&mut schema);

    if args.generate_tests {
        let types_path = args
            .tests_types_path
            .as_deref()
            .expect("clap guarantees --types-path when --tests is set");
        let tests_dir = args.generated_dir_path.join("tests");
        let tokens = generator::tests::tokenize_tests(&schema, types_path);
        output.write(&tokens, &tests_dir, "types_generated.rs");
        println!("Tests generated in one file");
    }

    let src_dir = args.generated_dir_path.join("src");
    let types_dir = src_dir.join("types");
    let known_schema_type_names = schema.types.keys().cloned().collect::<HashSet<_>>();
    let known_api_method_names = schema
        .methods
        .values()
        .map(|method| {
            (
                struct_name_to_method_name(&method.name),
                method.name.clone(),
            )
        })
        .collect::<std::collections::HashMap<_, _>>();
    for (name, ty) in &schema.types {
        let tokens = generator::types::tokenize_type(
            ty,
            &schema,
            &known_schema_type_names,
            &known_api_method_names,
        );
        output.write(&tokens, &types_dir, &camel_to_filename(name, Some("rs")));
    }
    println!("Types generated");

    let type_names = schema.types.keys().collect::<Vec<_>>();
    let tokens = generator::types::tokenize_types_mod(type_names.as_slice());
    output.write(&tokens, &src_dir, "types.rs");
    println!("Types module generated");

    generate_enums(&schema, &src_dir, &mut output);
    println!("Enums generated");

    let methods_dir = src_dir.join("methods");
    for method in schema.methods.values() {
        let tokens = generator::methods::tokenize_method(
            method,
            &known_schema_type_names,
            &known_api_method_names,
        );
        output.write(
            &tokens,
            &methods_dir,
            &camel_to_filename(&method.name, Some("rs")),
        );
    }
    println!("Methods generated");

    let to_methods_dir = types_dir.join("to_methods");
    let mut to_methods_type_names = vec![];
    for (name, tokens) in &generator::types_helpers::to_methods::tokenize_to_methods_files(&schema)
    {
        output.write(
            tokens,
            &to_methods_dir,
            &camel_to_filename(name, Some("rs")),
        );
        to_methods_type_names.push(*name);
    }
    let tokens =
        generator::types_helpers::to_methods::tokenize_to_methods_mod(&to_methods_type_names);
    output.write(&tokens, &types_dir, "to_methods.rs");
    println!("Type-to-methods helpers generated");

    let filters_dir = src_dir.join("filters");
    let tokens = generator::types_helpers::smart_filter::tokenize_smart_filter(&schema);
    output.write(&tokens, &filters_dir, "smart.rs");
    println!("Smart filters generated");

    let method_names = schema.methods.values().map(|m| &m.name).collect::<Vec<_>>();
    let tokens = generator::methods::tokenize_methods_mod(method_names.as_slice());
    output.write(&tokens, &src_dir, "methods.rs");
    println!("Methods module generated");
    output.finish();
}
