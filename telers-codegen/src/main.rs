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

    /// Rust path used in generated tests import: use <path>::types::*;
    #[arg(long, visible_alias = "types-path")]
    tests_types_path: String,
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
    schema.reorder_untagged_subtypes();

    if args.generate_tests {
        let tests_dir = args.generated_dir_path.join("tests");
        let tokens = generator::tests::tokenize_tests(&schema, &args.tests_types_path);
        let filename = "generated.rs";
        write_tokens_to_file(&tokens, &tests_dir, filename).unwrap_or_else(|err| {
            eprintln!("Failed to write file '{filename}' in dir tests: {err}\nContent: {tokens}");
            process::exit(1);
        });
        println!("Tests generated in one file");
    }

    let types_dir = args.generated_dir_path.join("src/types");
    for (name, ty) in &schema.types {
        let tokens = generator::types::tokenize_type(ty, &schema);
        let filename = camel_to_filename(name, Some("rs"));
        write_tokens_to_file(&tokens, &types_dir, &filename).unwrap_or_else(|err| {
            eprintln!("Failed to write file '{filename}' in dir types: {err}\nContent: {tokens}");
            process::exit(1);
        });
    }
    println!("Types generated");

    let type_names = schema.types.keys().collect::<Vec<_>>();
    let tokens = generator::types::tokenize_types_mod(type_names.as_slice());
    write_tokens_to_file(&tokens, &args.generated_dir_path, "types.rs").unwrap_or_else(|err| {
        eprintln!("Failed to write file 'types.rs': {err}\nContent: {tokens}");
        process::exit(1);
    });
    println!("Types module generated");
}
