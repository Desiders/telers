use clap::Parser;
use std::{fs, path::PathBuf, process};
use telers_codegen::{
    file::{camel_to_filename, write_tokens_to_file},
    generator::{self},
    parser::api::Schema,
};

#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    /// Path to generated directory
    #[arg(long, default_value = "./generated")]
    generated_dir_path: PathBuf,

    /// Path to schema JSON file
    #[arg(long, value_name = "PATH")]
    schema_json_path: PathBuf,
}

fn main() {
    let args = Args::parse();

    let schema_content = fs::read_to_string(&args.schema_json_path).unwrap_or_else(|err| {
        eprintln!(
            "Failed to read schema file '{}': {err}",
            args.schema_json_path.display(),
        );
        process::exit(1);
    });

    let schema = Schema::parse_from_jsom(&schema_content)
        .unwrap_or_else(|err| {
            eprintln!("Failed to parse schema file: {err}");
            process::exit(1);
        })
        .normalize();

    for (name, ty) in &schema.types {
        let tokens = generator::types::tokenize_type(ty, &schema);
        let filename = camel_to_filename(name, Some("rs"));
        write_tokens_to_file(&tokens, &args.generated_dir_path.join("types"), &filename)
            .unwrap_or_else(|err| {
                eprintln!(
                    "Failed to write file '{filename}' in dir types: {err}\nContent: {tokens}"
                );
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
