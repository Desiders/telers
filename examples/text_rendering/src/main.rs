use telers::{
    enums::UpdateType,
    event::telegram::{Handler, HandlerResult},
    methods::SendMessage,
    types::Message,
    Bot, Dispatcher, Router,
};

async fn handler(bot: Bot, message: Message) -> HandlerResult<()> {
    if message.text().is_none() {
        bot.send(SendMessage::new(
            message.chat().id(),
            "Send me a text message with some formatting",
        ))
        .await?;

        return Ok(());
    };

    if let Some(markdown) = message.html_text() {
        bot.send(SendMessage::new(
            message.chat().id(),
            format!("HTML source:\n\n{markdown}"),
        ))
        .await?;
    }
    if let Some(markdown) = message.markdown_text() {
        bot.send(SendMessage::new(
            message.chat().id(),
            format!("MarkdownV2 source:\n\n{markdown}"),
        ))
        .await?;
    }
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let router =
        Router::new("main").on_message(|observer| observer.register(Handler::new(handler)));

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
