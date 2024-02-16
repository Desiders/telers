use std::fs;
use telers_codegen::parser::api::parse_json_to_scheme;

#[test]
fn test_parse_json_to_scheme() {
    let content = match fs::read_to_string("api.json") {
        Ok(content) => content,
        Err(err) => panic!("Failed to read file: {err}"),
    };

    match parse_json_to_scheme(&content) {
        Ok(_scheme) => {}
        Err(err) => panic!("Failed to parse content: {err}"),
    };
}
