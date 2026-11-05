use telers_types::enums::ChatType;

#[test]
fn enum_parse_errors_work_with_standard_error_handling() {
    fn parse(value: &str) -> Result<ChatType, Box<dyn std::error::Error>> {
        Ok(value.parse()?)
    }

    assert_eq!(parse("private").unwrap(), ChatType::Private);
    assert!(parse("not-a-chat-type").is_err());
}
