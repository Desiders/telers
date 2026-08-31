use telers::{
    client::Reqwest,
    context::Context,
    filters::CommandObject,
    types::{ChatPrivate, MessageText, Update, UpdateMessage},
    Bot, Command, Extensions, Extractor, Request,
};

use std::sync::Arc;

#[derive(Clone, Debug, Command)]
#[command(rename_rule = "snake_case")]
enum Commands {
    #[command(description = "display this text")]
    Help,
    #[command(description = "handle a username")]
    Username(String),
    #[command(description = "handle a username and an age")]
    UsernameAndAge { username: String, age: u8 },
}

fn request_with_command(command: Option<&str>) -> Request {
    let mut context = Context::new();
    if let Some(command) = command {
        context.insert("command", CommandObject::extract(command).unwrap());
    }

    Request {
        bot: Bot::default(),
        update: Arc::new(Update::Message(UpdateMessage::new(
            0,
            MessageText::new(0, 0, ChatPrivate::new(0), ""),
        ))),
        context,
        extensions: Extensions::default(),
    }
}

fn extract<T>(request: &Request) -> Result<T, anyhow::Error>
where
    T: Extractor<Reqwest>,
    T::Error: Into<anyhow::Error>,
{
    tokio_test::block_on(<T as Extractor<Reqwest>>::extract(request)).map_err(Into::into)
}

#[test]
fn test_extract_unit_variant() {
    let request = request_with_command(Some("/help some extra args"));

    let Commands::Help = extract(&request).unwrap() else {
        panic!("expected `Help` variant")
    };
}

#[test]
fn test_extract_case_insensitive() {
    let request = request_with_command(Some("/HELP"));

    let Commands::Help = extract(&request).unwrap() else {
        panic!("expected `Help` variant")
    };
}

#[test]
fn test_extract_tuple_variant() {
    let request = request_with_command(Some("/username 42"));

    let Commands::Username(username) = extract(&request).unwrap() else {
        panic!("expected `Username` variant")
    };
    assert_eq!(username, "42");
}

#[test]
fn test_extract_named_variant() {
    let request = request_with_command(Some("/username_and_age 42 25"));

    let Commands::UsernameAndAge {
        username,
        age,
    } = extract(&request).unwrap()
    else {
        panic!("expected `UsernameAndAge` variant")
    };
    assert_eq!(username, "42");
    assert_eq!(age, 25);
}

#[test]
fn test_extract_error_not_enough_arguments() {
    let request = request_with_command(Some("/username"));

    let err = extract::<Commands>(&request).unwrap_err();
    assert!(err.to_string().contains("Not enough arguments"));
}

#[test]
fn test_extract_error_wrong_argument_type() {
    let request = request_with_command(Some("/username_and_age 42 not_a_number"));

    let err = extract::<Commands>(&request).unwrap_err();
    assert!(err.to_string().contains("Failed to parse `u8`"));
}

#[test]
fn test_extract_error_unknown_command() {
    let request = request_with_command(Some("/unknown"));

    let err = extract::<Commands>(&request).unwrap_err();
    assert!(err.to_string().contains("Unknown command"));
}

#[test]
fn test_extract_error_no_command_in_context() {
    let request = request_with_command(None);

    let err = extract::<Commands>(&request).unwrap_err();
    assert!(err.to_string().contains("Command` filter must be used"));
}

#[test]
fn test_descriptions() {
    assert_eq!(
        Commands::descriptions(),
        "/help - display this text\n/username - handle a username\n/username_and_age - handle a \
         username and an age"
    );
}

#[test]
fn test_bot_commands() {
    let commands = Commands::bot_commands();
    assert_eq!(commands.len(), 3);
    assert_eq!(commands[0].command.as_ref(), "help");
    assert_eq!(commands[0].description.as_ref(), "display this text");
    assert_eq!(commands[2].command.as_ref(), "username_and_age");
    assert_eq!(
        commands[2].description.as_ref(),
        "handle a username and an age"
    );
}

#[derive(Clone, Debug, Command)]
#[command(rename_rule = "camel_case", prefix = "!")]
enum VariedCommands {
    #[command(description = "hidden help", hidden)]
    HelpMe,
    #[command(description = "start", aliases = ["go", "begin"])]
    Start,
    #[command(rename = "do-it", description = "custom name")]
    DoIt,
    #[command(description = "parsed", parse_with = "parse_username")]
    Parsed(String),
    #[command(description = "name and age")]
    NameAndAge { name: String, age: u8 },
}

fn parse_username(args: &str) -> Result<VariedCommands, &'static str> {
    if args.is_empty() {
        Err("empty args")
    } else {
        Ok(VariedCommands::Parsed(args.to_owned()))
    }
}

#[test]
fn test_v2_camel_case_rename_rule() {
    let request = request_with_command(Some("!helpMe"));
    assert!(matches!(extract(&request).unwrap(), VariedCommands::HelpMe));
}

#[test]
fn test_v2_enum_level_prefix() {
    let request = request_with_command(Some("/helpMe"));
    let err = extract::<VariedCommands>(&request).unwrap_err();
    assert!(err.to_string().contains("Unknown command"));
}

#[test]
fn test_v2_rename_attr() {
    let request = request_with_command(Some("!do-it"));
    assert!(matches!(extract(&request).unwrap(), VariedCommands::DoIt));

    let request = request_with_command(Some("!doIt"));
    assert!(extract::<VariedCommands>(&request)
        .unwrap_err()
        .to_string()
        .contains("Unknown command"));
}

#[test]
fn test_v2_aliases() {
    for command in ["!start", "!go", "!begin"] {
        let request = request_with_command(Some(command));
        assert!(
            matches!(extract(&request).unwrap(), VariedCommands::Start),
            "expected `Start` for `{command}`"
        );
    }
}

#[test]
fn test_v2_hidden_excluded_from_lists_but_matchable() {
    let descriptions = VariedCommands::descriptions();
    assert!(!descriptions.contains("helpMe"));
    assert_eq!(
        descriptions,
        "/start - start\n/do-it - custom name\n/parsed - parsed\n/nameAndAge - name and age"
    );

    let commands = VariedCommands::bot_commands();
    assert_eq!(commands.len(), 4);
    assert!(!commands
        .iter()
        .any(|command| command.command.as_ref() == "helpMe"));

    let request = request_with_command(Some("!helpMe"));
    assert!(matches!(extract(&request).unwrap(), VariedCommands::HelpMe));
}

#[test]
fn test_v2_variant_level_parse_with() {
    let request = request_with_command(Some("!parsed hello world"));
    let VariedCommands::Parsed(parsed) = extract(&request).unwrap() else {
        panic!("expected `Parsed` variant")
    };
    assert_eq!(parsed, "hello world");
}

#[test]
fn test_v2_parse_with_error() {
    let request = request_with_command(Some("!parsed"));
    let err = extract::<VariedCommands>(&request).unwrap_err();
    assert!(err
        .to_string()
        .contains("Failed to parse arguments for `parsed` command"));
    assert!(err.to_string().contains("empty args"));
}

#[derive(Clone, Debug, Command)]
#[command(parse_with = "parse_all")]
enum FallbackCommands {
    #[command(description = "a")]
    A,
    #[command(description = "b", parse_with = "parse_b")]
    B,
}

fn parse_all(args: &str) -> Result<FallbackCommands, &'static str> {
    if args.is_empty() {
        Ok(FallbackCommands::A)
    } else {
        Err("unexpected args")
    }
}

fn parse_b(args: &str) -> Result<FallbackCommands, &'static str> {
    if args == "42" {
        Ok(FallbackCommands::B)
    } else {
        Err("expected 42")
    }
}

#[test]
fn test_v2_enum_level_parse_with() {
    let request = request_with_command(Some("/a"));
    assert!(matches!(extract(&request).unwrap(), FallbackCommands::A));
}

#[test]
fn test_v2_variant_parse_with_overrides_enum() {
    let request = request_with_command(Some("/b 42"));
    assert!(matches!(extract(&request).unwrap(), FallbackCommands::B));

    let request = request_with_command(Some("/b 7"));
    let err = extract::<FallbackCommands>(&request).unwrap_err();
    assert!(err.to_string().contains("expected 42"));
}
