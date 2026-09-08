use regex::Regex;
use telers::{
    enums::UpdateType,
    errors::{EventError, EventErrorKind, HandlerError},
    event::telegram::{Handler, HandlerResult},
    filters::{Command, CommandObject, ErrorMessage, ErrorType},
    methods::SendMessage,
    types::Message,
    Bot, Dispatcher, Router,
};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
#[error("Invalid age: {0}")]
struct InvalidAge(&'static str);

#[derive(Debug, Error)]
#[error("Invalid name: {0}")]
struct InvalidName(&'static str);

async fn set_age(bot: Bot, message: Message, command: CommandObject) -> HandlerResult<()> {
    let Some(age) = command.args.first() else {
        return Err(HandlerError::new(InvalidAge("no age provided")));
    };
    let Ok(age) = age.parse::<u8>() else {
        return Err(HandlerError::new(InvalidAge("age should be a number")));
    };

    bot.send(SendMessage::new(
        message.chat().id(),
        format!("Your age is {age}"),
    ))
    .await?;
    Ok(())
}

async fn set_name(bot: Bot, message: Message, command: CommandObject) -> HandlerResult<()> {
    let Some(name) = command.args.first() else {
        return Err(HandlerError::new(InvalidName("no name provided")));
    };

    bot.send(SendMessage::new(
        message.chat().id(),
        format!("Your name is {name}"),
    ))
    .await?;
    Ok(())
}

async fn on_invalid_age(
    bot: Bot,
    message: Message,
    EventError(err): EventError<InvalidAge>,
) -> HandlerResult<()> {
    bot.send(SendMessage::new(
        message.chat().id(),
        format!("Error caught: {err}"),
    ))
    .await?;
    Ok(())
}

/// Handles the errors whose text starts with `Invalid`
async fn on_invalid(bot: Bot, message: Message, err: EventErrorKind) -> HandlerResult<()> {
    bot.send(SendMessage::new(
        message.chat().id(),
        format!("Error caught: {err}"),
    ))
    .await?;
    Ok(())
}

/// Logs any other error, for example a failed request to the Telegram API
async fn on_error(err: EventErrorKind) -> HandlerResult<()> {
    tracing::error!(error = %err, "Unexpected error");
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let router = Router::new("main")
        .on_message(|observer| {
            observer.registers([
                Handler::new(set_age).filter(Command::one("age")),
                Handler::new(set_name).filter(Command::one("name")),
            ])
        })
        .on_error(|observer| {
            observer.registers([
                // Only `InvalidAge` errors
                Handler::new(on_invalid_age).filter(ErrorType::<InvalidAge>::new()),
                // Other errors whose text starts with `Invalid`, so `InvalidName` errors
                Handler::new(on_invalid).filter(ErrorMessage::one(
                    Regex::new("^Invalid").expect("the regex is valid"),
                )),
                // Anything else
                Handler::new(on_error),
            ])
        });

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        .allowed_update(UpdateType::Message)
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => tracing::info!("Bot stopped"),
        Err(err) => tracing::error!(error = %err, "Bot stopped"),
    }
}
