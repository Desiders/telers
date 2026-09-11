use telers::{
    enums::UpdateType,
    errors::EventError,
    event::telegram::{Handler, HandlerResult},
    filters::{Command as CommandFilter, ErrorType},
    methods::{SendMessage, SetMyCommands},
    types::Message,
    utils::command_args::{ArgsCursor, CommandArg, CommandArgsError},
    Bot, Command, Dispatcher, Router,
};

#[derive(Clone, Command)]
#[command(rename_rule = "snake_case")]
enum Commands {
    #[command(description = "display this text")]
    Help,
    #[command(description = "handle a username")]
    Username(String),
    #[command(description = "handle a username and an age")]
    UsernameAndAge { username: String, age: u8 },
    #[command(
        description = "handle settings in the `key=value,key=value` format",
        split = ','
    )]
    Settings(Settings),
}

/// Settings in the `key=value` format separated by commas,
/// for example `/settings lang=en, notifications=off`
#[derive(Clone)]
struct Settings(Vec<(String, String)>);

impl CommandArg for Settings {
    fn parse_arg(cursor: &mut ArgsCursor<'_>) -> Result<Self, CommandArgsError> {
        if cursor.is_exhausted() {
            return Err(CommandArgsError::Missing {
                index: 0,
            });
        }

        let mut settings = Vec::new();

        while let Some(arg) = cursor.next_arg() {
            let (key, value) = arg.split_once('=').ok_or_else(|| {
                CommandArgsError::from_display(format!("Expected `key=value`, got `{arg}`"))
            })?;

            settings.push((key.to_owned(), value.to_owned()));
        }

        Ok(Self(settings))
    }
}

async fn help_handler(bot: Bot, message: Message) -> HandlerResult<()> {
    bot.send(SendMessage::new(
        message.chat().id(),
        Commands::descriptions(),
    ))
    .await?;
    Ok(())
}

async fn username_handler(bot: Bot, message: Message, command: Commands) -> HandlerResult<()> {
    let text = match command {
        Commands::Username(username) => format!("Your username is {username}"),
        Commands::UsernameAndAge {
            username,
            age,
        } => {
            format!("Your username is {username}, age is {age}")
        }
        _ => return Ok(()),
    };

    bot.send(SendMessage::new(message.chat().id(), text))
        .await?;
    Ok(())
}

async fn settings_handler(bot: Bot, message: Message, command: Commands) -> HandlerResult<()> {
    let Commands::Settings(Settings(settings)) = command else {
        return Ok(());
    };

    let text = settings
        .iter()
        .map(|(key, value)| format!("{key} = {value}"))
        .collect::<Vec<_>>()
        .join("\n");

    bot.send(SendMessage::new(message.chat().id(), text))
        .await?;
    Ok(())
}

async fn on_command_args_error(
    bot: Bot,
    message: Message,
    EventError(err): EventError<CommandArgsError>,
) -> HandlerResult<()> {
    bot.send(SendMessage::new(message.chat().id(), err.to_string()))
        .await?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    if let Err(err) = bot.send(SetMyCommands::new(Commands::bot_commands())).await {
        tracing::error!(error = %err, "Failed to set commands");
    }

    let router = Router::new("main")
        .on_message(|observer| {
            observer
                .register(Handler::new(help_handler).filter(CommandFilter::one(CommandsType::Help)))
        })
        .on_message(|observer| {
            observer.register(Handler::new(username_handler).filter(CommandFilter::many([
                CommandsType::Username,
                CommandsType::UsernameAndAge,
            ])))
        })
        .on_message(|observer| {
            observer.register(
                Handler::new(settings_handler).filter(CommandFilter::one(CommandsType::Settings)),
            )
        })
        .on_error(|observer| {
            observer.register(
                Handler::new(on_command_args_error).filter(ErrorType::<CommandArgsError>::new()),
            )
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
