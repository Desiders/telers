use clap::Parser;
use std::{fs, path::PathBuf, process};
use telers_codegen::{
    file::{camel_to_filename, write_tokens_to_file},
    generator,
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
    #[arg(long, visible_alias = "tests", default_value_t = false)]
    generate_tests: bool,

    /// Rust path used in generated tests import: `use <path>::types::*;`
    #[arg(long, visible_alias = "types-path")]
    tests_types_path: String,
}

#[allow(clippy::too_many_lines)]
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
    schema.split_giveaway_types();
    schema.split_giveaway_winners_types();
    schema.split_star_transaction_types();
    schema.split_encrypted_passport_element_types();
    schema.split_message_entity_types();
    schema.split_inline_query_result();
    schema.split_transaction_partner_user_types();
    schema.compose_reply_markup_type();
    schema.reorder_untagged_subtypes();
    schema.modify_get_updates_returns_method();

    if args.generate_tests {
        let tests_dir = args.generated_dir_path.join("tests");
        let tokens = generator::tests::tokenize_tests(&schema, &args.tests_types_path);
        let filename = "types_generated.rs";
        write_tokens_to_file(&tokens, &tests_dir, filename).unwrap_or_else(|err| {
            eprintln!("Failed to write file '{filename}' in dir tests: {err}\nContent: {tokens}");
            process::exit(1);
        });
        println!("Tests generated in one file");
    }

    let src_dir = args.generated_dir_path.join("src");
    let types_dir = src_dir.join("types");
    let known_schema_type_names = schema
        .types
        .keys()
        .cloned()
        .collect::<std::collections::HashSet<_>>();
    for (name, ty) in &schema.types {
        let tokens = generator::types::tokenize_type(ty, &schema, &known_schema_type_names);
        let filename = camel_to_filename(name, Some("rs"));
        write_tokens_to_file(&tokens, &types_dir, &filename).unwrap_or_else(|err| {
            eprintln!("Failed to write file '{filename}' in dir types: {err}\nContent: {tokens}");
            process::exit(1);
        });
    }
    println!("Types generated");

    let type_names = schema.types.keys().collect::<Vec<_>>();
    let tokens = generator::types::tokenize_types_mod(type_names.as_slice());
    write_tokens_to_file(&tokens, &src_dir, "types.rs").unwrap_or_else(|err| {
        eprintln!("Failed to write file 'types.rs': {err}\nContent: {tokens}");
        process::exit(1);
    });
    println!("Types module generated");

    let enums_dir = src_dir.join("enums");
    let enum_names = schema
        .types
        .iter()
        .filter(|(_, ty)| !ty.subtypes.is_empty())
        .filter_map(|(name, ty)| {
            let tokens = generator::enums::tokenize_kind_enum_file(ty)?;
            let filename = camel_to_filename(name, Some("rs"));
            write_tokens_to_file(&tokens, &enums_dir, &filename).unwrap_or_else(|err| {
                eprintln!(
                    "Failed to write file '{filename}' in dir enums: {err}\nContent: {tokens}"
                );
                process::exit(1);
            });
            Some(name.as_str())
        })
        .collect::<Vec<_>>();
    let tokens_with_names = generator::enums::tokenize_own_enums();
    let mut own_enum_names = vec![];
    for (name, tokens) in &tokens_with_names {
        let filename = camel_to_filename(name, Some("rs"));
        write_tokens_to_file(tokens, &enums_dir, &filename).unwrap_or_else(|err| {
            eprintln!("Failed to write file '{filename}' in dir enums: {err}\nContent: {tokens}");
            process::exit(1);
        });
        own_enum_names.push(*name);
    }
    let tokens =
        generator::enums::tokenize_kind_enums_mod(enum_names.as_slice(), own_enum_names.as_slice());
    write_tokens_to_file(&tokens, &src_dir, "enums.rs").unwrap_or_else(|err| {
        eprintln!("Failed to write file 'enums.rs': {err}\nContent: {tokens}");
        process::exit(1);
    });
    println!("Enums generated");

    let methods_dir = src_dir.join("methods");
    let known_api_method_names = schema
        .methods
        .values()
        .map(|m| {
            let mut chars = m.name.chars();
            let api_name = match chars.next() {
                Some(first) => first.to_lowercase().chain(chars).collect::<String>(),
                None => String::new(),
            };
            (api_name, m.name.clone())
        })
        .collect::<std::collections::HashMap<_, _>>();
    for method in schema.methods.values() {
        let tokens = generator::methods::tokenize_method(
            method,
            &known_schema_type_names,
            &known_api_method_names,
        );
        let filename = camel_to_filename(&method.name, Some("rs"));
        write_tokens_to_file(&tokens, &methods_dir, &filename).unwrap_or_else(|err| {
            eprintln!("Failed to write file '{filename}' in dir methods: {err}\nContent: {tokens}");
            process::exit(1);
        });
    }
    println!("Methods generated");

    let method_names = schema.methods.values().map(|m| &m.name).collect::<Vec<_>>();
    let tokens = generator::methods::tokenize_methods_mod(method_names.as_slice());
    write_tokens_to_file(&tokens, &src_dir, "methods.rs").unwrap_or_else(|err| {
        eprintln!("Failed to write file 'methods.rs': {err}\nContent: {tokens}");
        process::exit(1);
    });
    println!("Methods module generated");
}
