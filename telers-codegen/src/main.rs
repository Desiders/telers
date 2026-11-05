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

    /// Write API types, enums and their tests to a separate crate directory
    #[arg(long = "types-gen-dir", value_name = "PATH")]
    types_generated_dir_path: Option<PathBuf>,

    /// Write API methods and client-independent request builders to a separate crate directory
    #[arg(long = "methods-gen-dir", value_name = "PATH")]
    methods_generated_dir_path: Option<PathBuf>,

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
    framework_src_dir: Option<&Path>,
) {
    let enums_dir = src_dir.join("enums");
    let mut enum_names = schema
        .types
        .iter()
        .filter(|(_, ty)| !ty.subtypes.is_empty())
        .filter_map(|(name, ty)| {
            let tokens = generator::enums::tokenize_kind_enum_file(ty)?;
            output.write(&tokens, &enums_dir, &camel_to_filename(name, Some("rs")));
            Some(name.as_str())
        })
        .collect::<Vec<_>>();
    enum_names.sort_unstable();
    let mut own_enum_names = vec![];
    for (name, tokens) in &generator::enums::tokenize_own_enums(schema) {
        if *name == "TelegramObserverType"
            && let Some(framework_src_dir) = framework_src_dir
        {
            output.write(
                tokens,
                &framework_src_dir.join("enums"),
                "telegram_observer_type.rs",
            );
            continue;
        }
        output.write(tokens, &enums_dir, &camel_to_filename(name, Some("rs")));
        own_enum_names.push(*name);
    }
    let tokens = if framework_src_dir.is_some() {
        generator::enums::tokenize_kind_enums_mod_without_framework(
            enum_names.as_slice(),
            own_enum_names.as_slice(),
        )
    } else {
        generator::enums::tokenize_kind_enums_mod(enum_names.as_slice(), own_enum_names.as_slice())
    };
    output.write(&tokens, src_dir, "enums.rs");
}

fn generate_shortcuts(
    schema: &telers_codegen::parser::api::NormalizedSchema,
    src_dir: &Path,
    split_types: bool,
    output: &mut Output,
) {
    let framework_types_dir = src_dir.join("types");
    let to_methods_dir = framework_types_dir.join("to_methods");
    let mut to_methods_type_names = vec![];
    let tokenize_shortcuts = if split_types {
        generator::types_helpers::to_methods::tokenize_to_methods_files_for_external_types
    } else {
        generator::types_helpers::to_methods::tokenize_to_methods_files
    };
    for (name, tokens) in &tokenize_shortcuts(schema) {
        output.write(
            tokens,
            &to_methods_dir,
            &camel_to_filename(name, Some("rs")),
        );
        to_methods_type_names.push(*name);
    }
    let tokens = if split_types {
        generator::types_helpers::to_methods::tokenize_to_methods_mod_for_external_types(
            &to_methods_type_names,
        )
    } else {
        generator::types_helpers::to_methods::tokenize_to_methods_mod(&to_methods_type_names)
    };
    output.write(&tokens, &framework_types_dir, "to_methods.rs");
}

fn generate_methods(
    schema: &telers_codegen::parser::api::NormalizedSchema,
    src_dir: &Path,
    known_schema_type_names: &HashSet<String>,
    known_api_method_names: &std::collections::HashMap<String, String>,
    output: &mut Output,
    standalone: bool,
) {
    let methods_dir = src_dir.join("methods");
    let tokenize_method = if standalone {
        generator::methods::tokenize_method_without_client
    } else {
        generator::methods::tokenize_method
    };
    for method in schema.methods.values() {
        let tokens = tokenize_method(method, known_schema_type_names, known_api_method_names);
        output.write(
            &tokens,
            &methods_dir,
            &camel_to_filename(&method.name, Some("rs")),
        );
    }
    let mut method_names = schema.methods.values().map(|m| &m.name).collect::<Vec<_>>();
    method_names.sort_unstable();
    let tokens = if standalone {
        generator::methods::tokenize_methods_mod_without_client(&method_names)
    } else {
        generator::methods::tokenize_methods_mod(&method_names)
    };
    output.write(&tokens, src_dir, "methods.rs");
}

fn read_schema(path: &Path) -> telers_codegen::parser::api::NormalizedSchema {
    let schema_content = fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!("Failed to read schema file '{}': {err}", path.display());
        process::exit(1);
    });

    let mut schema = Schema::parse_from_json(&schema_content)
        .unwrap_or_else(|err| {
            eprintln!("Failed to parse schema file: {err}");
            process::exit(1);
        })
        .normalize();

    apply_schema_transforms(&mut schema);
    schema
}

fn main() {
    let args = Args::parse();
    let mut output = Output::new(args.check);
    let split_types = args.types_generated_dir_path.is_some();
    let split_methods = args.methods_generated_dir_path.is_some();
    let types_crate_dir = args
        .types_generated_dir_path
        .as_ref()
        .unwrap_or(&args.generated_dir_path);
    let methods_crate_dir = args
        .methods_generated_dir_path
        .as_ref()
        .unwrap_or(&args.generated_dir_path);
    let schema = read_schema(&args.schema_json_path);

    if args.generate_tests {
        let types_path = args
            .tests_types_path
            .as_deref()
            .expect("clap guarantees --types-path when --tests is set");
        let tests_dir = types_crate_dir.join("tests");
        let tokens = generator::tests::tokenize_tests(&schema, types_path);
        output.write(&tokens, &tests_dir, "types_generated.rs");
        println!("Tests generated in one file");
    }

    let src_dir = args.generated_dir_path.join("src");
    let types_src_dir = types_crate_dir.join("src");
    let methods_src_dir = methods_crate_dir.join("src");
    let types_dir = types_src_dir.join("types");
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
        let tokenize_type = if split_types {
            generator::types::tokenize_type_without_extractors
        } else {
            generator::types::tokenize_type
        };
        let tokens = tokenize_type(
            ty,
            &schema,
            &known_schema_type_names,
            &known_api_method_names,
        );
        output.write(&tokens, &types_dir, &camel_to_filename(name, Some("rs")));
    }
    let mut type_names = schema.types.keys().collect::<Vec<_>>();
    type_names.sort_unstable();
    let tokens = if split_types {
        generator::types::tokenize_types_mod_without_framework(type_names.as_slice())
    } else {
        generator::types::tokenize_types_mod(type_names.as_slice())
    };
    output.write(&tokens, &types_src_dir, "types.rs");
    println!("Types module generated");

    generate_enums(
        &schema,
        &types_src_dir,
        &mut output,
        split_types.then_some(src_dir.as_path()),
    );
    println!("Enums generated");

    if split_types {
        let tokens = generator::types::tokenize_type_extractors(&schema);
        output.write(&tokens, &src_dir, "type_extractors.rs");
        println!("Framework type extractors generated");
    }

    generate_methods(
        &schema,
        &methods_src_dir,
        &known_schema_type_names,
        &known_api_method_names,
        &mut output,
        split_methods,
    );
    println!("Methods generated");

    generate_shortcuts(&schema, &src_dir, split_types, &mut output);
    println!("Type-to-methods helpers generated");

    let filters_dir = src_dir.join("filters");
    let tokens = generator::types_helpers::smart_filter::tokenize_smart_filter(&schema);
    output.write(&tokens, &filters_dir, "smart.rs");
    println!("Smart filters generated");

    output.finish();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn types_destination_is_optional_for_legacy_generation() {
        let legacy = Args::try_parse_from([
            "telers-codegen",
            "--schema",
            "api.json",
            "--gen-dir",
            "telers",
        ])
        .unwrap();
        assert!(legacy.types_generated_dir_path.is_none());
        assert!(legacy.methods_generated_dir_path.is_none());
        let split = Args::try_parse_from([
            "telers-codegen",
            "--schema",
            "api.json",
            "--gen-dir",
            "telers",
            "--types-gen-dir",
            "telers-types",
            "--methods-gen-dir",
            "telers-methods",
            "--tests",
            "--types-path",
            "telers_types",
        ])
        .unwrap();
        assert_eq!(
            split.types_generated_dir_path,
            Some(PathBuf::from("telers-types"))
        );
        assert_eq!(split.tests_types_path.as_deref(), Some("telers_types"));
        assert_eq!(
            split.methods_generated_dir_path,
            Some(PathBuf::from("telers-methods"))
        );
    }

    #[test]
    fn standalone_enums_leave_observer_in_the_framework() {
        let mut schema = Schema::parse_from_json(include_str!("../schemas/api.json"))
            .unwrap()
            .normalize();
        apply_schema_transforms(&mut schema);
        let root =
            std::env::temp_dir().join(format!("telers-codegen-split-{}", std::process::id()));
        let types_src = root.join("types/src");
        let framework_src = root.join("framework/src");
        let mut output = Output::new(false);
        generate_enums(&schema, &types_src, &mut output, Some(&framework_src));
        assert!(
            framework_src
                .join("enums/telegram_observer_type.rs")
                .exists()
        );
        assert!(!types_src.join("enums/telegram_observer_type.rs").exists());
        let module = fs::read_to_string(types_src.join("enums.rs")).unwrap();
        assert!(module.contains("pub use parse_mode::ParseMode"));
        assert!(!module.contains("TelegramObserverType"));
        assert!(!module.contains("methods::SendMessage"));
        let mut check = Output::new(true);
        generate_enums(&schema, &types_src, &mut check, Some(&framework_src));
        assert!(check.drifted.is_empty());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn standalone_methods_check_their_separate_destination() {
        let mut schema = Schema::parse_from_json(include_str!("../schemas/api.json"))
            .unwrap()
            .normalize();
        schema
            .methods
            .retain(|name, _| matches!(name.as_str(), "sendMessage" | "sendPhoto"));
        let root =
            std::env::temp_dir().join(format!("telers-codegen-methods-{}", std::process::id()));
        let src_dir = root.join("methods/src");
        let mut output = Output::new(false);
        generate_methods(
            &schema,
            &src_dir,
            &HashSet::new(),
            &std::collections::HashMap::new(),
            &mut output,
            true,
        );
        let filename = src_dir.join("methods/send_photo.rs");
        let method = fs::read_to_string(&filename).unwrap();
        assert!(method.contains("fn build_request(mut self)"));
        assert!(!method.contains("crate::client"));
        assert!(src_dir.join("methods.rs").exists());
        let mut check = Output::new(true);
        generate_methods(
            &schema,
            &src_dir,
            &HashSet::new(),
            &std::collections::HashMap::new(),
            &mut check,
            true,
        );
        assert!(check.drifted.is_empty());
        fs::write(&filename, "// outdated\n").unwrap();
        generate_methods(
            &schema,
            &src_dir,
            &HashSet::new(),
            &std::collections::HashMap::new(),
            &mut check,
            true,
        );
        assert_eq!(check.drifted, vec![filename.to_string_lossy().into_owned()]);
        assert_eq!(fs::read_to_string(&filename).unwrap(), "// outdated\n");
        fs::remove_dir_all(root).unwrap();
    }
}
