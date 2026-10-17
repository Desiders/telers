use telers::{
    client::session::Session,
    enums::UpdateType,
    event::telegram::{Handler, HandlerResult},
    methods::SendMessage,
    types::Message,
    Bot, Dispatcher, Router,
};

async fn echo_handler(bot: Bot<impl Session>, message: Message) -> HandlerResult<()> {
    bot.send(SendMessage::new(message.chat().id(), "pong"))
        .await?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // The middleware reports every outgoing API call, so its events are worth seeing here.
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("info,telers=debug"))
        .init();

    let token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN env variable is not set!");
    // Polling reports every incoming update through `getUpdates`, so its log drowns out the rest
    let bot = Bot::new(token).on_middleware(telers::Logging::new().ignore_methods(["getUpdates"]));

    let router =
        Router::new("main").on_message(|observer| observer.register(Handler::new(echo_handler)));

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
