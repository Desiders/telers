use telers::{
    enums::UpdateType,
    event::telegram::{Handler, HandlerResult},
    filters::{command::Command as CommandFilter, CommandObject},
    methods::{SendMessage, SetMyCommands},
    types::Message,
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
}

async fn help_handler(bot: Bot, message: Message) -> HandlerResult<()> {
    bot.send(SendMessage::new(
        message.chat().id(),
        Commands::descriptions(),
    ))
    .await?;
    Ok(())
}

async fn username_handler(
    bot: Bot,
    message: Message,
    command: Commands,
    _command_object: CommandObject,
) -> HandlerResult<()> {
    let text = match command {
        Commands::Username(username) => format!("Your username is {username}"),
        Commands::UsernameAndAge {
            username,
            age,
        } => {
            format!("Your username is {username}, age is {age}")
        }
        Commands::Help => return Ok(()),
    };

    bot.send(SendMessage::new(message.chat().id(), text))
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
            observer.register(
                Handler::new(help_handler).filter(
                    CommandFilter::builder()
                        .command("help")
                        .ignore_case(true)
                        .build(),
                ),
            )
        })
        .on_message(|observer| {
            observer.register(
                Handler::new(username_handler).filter(
                    CommandFilter::builder()
                        .commands(["username", "username_and_age"])
                        .ignore_case(true)
                        .build(),
                ),
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
