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
    let request = request_with_command(Some("/help"));

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
    assert!(err.to_string().contains("Missing argument"));
}

#[test]
fn test_extract_error_wrong_argument_type() {
    let request = request_with_command(Some("/username_and_age 42 not_a_number"));

    let err = extract::<Commands>(&request).unwrap_err();
    assert!(err
        .to_string()
        .contains("Invalid value `not_a_number` for argument `age`"));
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
#[command(rename_rule = "snake_case", prefix = '!')]
enum VariedCommands {
    #[command(description = "hidden help", hidden)]
    HelpMe,
    #[command(description = "start", aliases = ["go", "begin"])]
    Start,
    #[command(rename = "do-it", description = "custom name")]
    DoIt,
    #[command(description = "name and age")]
    NameAndAge { name: String, age: u8 },
}

#[test]
fn test_snake_case_rename_rule() {
    let request = request_with_command(Some("!help_me"));
    assert!(matches!(extract(&request).unwrap(), VariedCommands::HelpMe));
}

#[test]
fn test_enum_level_prefix() {
    let request = request_with_command(Some("/help_me"));
    let err = extract::<VariedCommands>(&request).unwrap_err();
    assert!(err.to_string().contains("Unknown command"));
}

#[test]
fn test_rename_attr() {
    let request = request_with_command(Some("!do-it"));
    assert!(matches!(extract(&request).unwrap(), VariedCommands::DoIt));

    let request = request_with_command(Some("!doIt"));
    assert!(extract::<VariedCommands>(&request)
        .unwrap_err()
        .to_string()
        .contains("Unknown command"));
}

#[test]
fn test_aliases() {
    for command in ["!start", "!go", "!begin"] {
        let request = request_with_command(Some(command));
        assert!(
            matches!(extract(&request).unwrap(), VariedCommands::Start),
            "expected `Start` for `{command}`"
        );
    }
}

#[test]
fn test_hidden_excluded_from_lists_but_matchable() {
    let descriptions = VariedCommands::descriptions();
    assert!(!descriptions.contains("help_me"));
    assert_eq!(
        descriptions,
        "/start - start\n/do-it - custom name\n/name_and_age - name and age"
    );

    let commands = VariedCommands::bot_commands();
    assert_eq!(commands.len(), 3);
    assert!(!commands
        .iter()
        .any(|command| command.command.as_ref() == "help_me"));

    let request = request_with_command(Some("!help_me"));
    assert!(matches!(extract(&request).unwrap(), VariedCommands::HelpMe));
}
