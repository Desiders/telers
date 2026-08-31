use telers::{
    client::Reqwest,
    context::Context,
    errors::ExtractionError,
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

fn extract(request: &Request) -> Result<Commands, ExtractionError> {
    tokio_test::block_on(<Commands as Extractor<Reqwest>>::extract(request))
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

    let err = extract(&request).unwrap_err();
    assert!(err.to_string().contains("Not enough arguments"));
}

#[test]
fn test_extract_error_wrong_argument_type() {
    let request = request_with_command(Some("/username_and_age 42 not_a_number"));

    let err = extract(&request).unwrap_err();
    assert!(err.to_string().contains("Failed to parse `u8`"));
}

#[test]
fn test_extract_error_unknown_command() {
    let request = request_with_command(Some("/unknown"));

    let err = extract(&request).unwrap_err();
    assert!(err.to_string().contains("Unknown command"));
}

#[test]
fn test_extract_error_no_command_in_context() {
    let request = request_with_command(None);

    let err = extract(&request).unwrap_err();
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
