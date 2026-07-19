use clap::Parser;
use std::{
    collections::HashSet,
    fmt::Display,
    fs,
    path::{Path, PathBuf},
    process,
};
use telers_codegen::{
    file::{camel_to_filename, write_tokens_to_file},
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
}

fn write_or_exit(tokens: &impl Display, dir: &Path, filename: &str) {
    write_tokens_to_file(tokens, dir, filename).unwrap_or_else(|err| {
        eprintln!(
            "Failed to write file '{filename}' in dir '{dir}': {err}\nContent: {tokens}",
            dir = dir.display()
        );
        process::exit(1);
    });
}

fn main() {
    let args = Args::parse();

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

    if args.generate_tests {
        let types_path = args
            .tests_types_path
            .as_deref()
            .expect("clap guarantees --types-path when --tests is set");
        let tests_dir = args.generated_dir_path.join("tests");
        let tokens = generator::tests::tokenize_tests(&schema, types_path);
        write_or_exit(&tokens, &tests_dir, "types_generated.rs");
        println!("Tests generated in one file");
    }

    let src_dir = args.generated_dir_path.join("src");
    let types_dir = src_dir.join("types");
    let known_schema_type_names = schema.types.keys().cloned().collect::<HashSet<_>>();
    for (name, ty) in &schema.types {
        let tokens = generator::types::tokenize_type(ty, &schema, &known_schema_type_names);
        write_or_exit(&tokens, &types_dir, &camel_to_filename(name, Some("rs")));
    }
    println!("Types generated");

    let type_names = schema.types.keys().collect::<Vec<_>>();
    let tokens = generator::types::tokenize_types_mod(type_names.as_slice());
    write_or_exit(&tokens, &src_dir, "types.rs");
    println!("Types module generated");

    let enums_dir = src_dir.join("enums");
    let enum_names = schema
        .types
        .iter()
        .filter(|(_, ty)| !ty.subtypes.is_empty())
        .filter_map(|(name, ty)| {
            let tokens = generator::enums::tokenize_kind_enum_file(ty)?;
            write_or_exit(&tokens, &enums_dir, &camel_to_filename(name, Some("rs")));
            Some(name.as_str())
        })
        .collect::<Vec<_>>();
    let mut own_enum_names = vec![];
    for (name, tokens) in &generator::enums::tokenize_own_enums(&schema) {
        write_or_exit(tokens, &enums_dir, &camel_to_filename(name, Some("rs")));
        own_enum_names.push(*name);
    }
    let tokens =
        generator::enums::tokenize_kind_enums_mod(enum_names.as_slice(), own_enum_names.as_slice());
    write_or_exit(&tokens, &src_dir, "enums.rs");
    println!("Enums generated");

    let methods_dir = src_dir.join("methods");
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
    for method in schema.methods.values() {
        let tokens = generator::methods::tokenize_method(
            method,
            &known_schema_type_names,
            &known_api_method_names,
        );
        write_or_exit(
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
        write_or_exit(
            tokens,
            &to_methods_dir,
            &camel_to_filename(name, Some("rs")),
        );
        to_methods_type_names.push(*name);
    }
    let tokens =
        generator::types_helpers::to_methods::tokenize_to_methods_mod(&to_methods_type_names);
    write_or_exit(&tokens, &types_dir, "to_methods.rs");
    println!("Type-to-methods helpers generated");

    let filters_dir = src_dir.join("filters");
    let tokens = generator::types_helpers::smart_filter::tokenize_smart_filter(&schema);
    write_or_exit(&tokens, &filters_dir, "smart.rs");
    println!("Smart filters generated");

    let method_names = schema.methods.values().map(|m| &m.name).collect::<Vec<_>>();
    let tokens = generator::methods::tokenize_methods_mod(method_names.as_slice());
    write_or_exit(&tokens, &src_dir, "methods.rs");
    println!("Methods module generated");
}
