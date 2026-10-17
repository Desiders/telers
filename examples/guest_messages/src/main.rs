use telers::{
    enums::UpdateType,
    event::telegram::{Handler, HandlerResult},
    methods::AnswerGuestQuery,
    types::{InlineQueryResultArticle, InputTextMessageContent, Message},
    Bot, Dispatcher, Router,
};

async fn guest_handler(bot: Bot, message: Message) -> HandlerResult<()> {
    let Some(guest_query_id) = message.guest_query_id() else {
        return Ok(());
    };

    bot.send(AnswerGuestQuery::new(
        guest_query_id,
        InlineQueryResultArticle::new(
            "guest-reply",
            "Guest reply",
            InputTextMessageContent::new("Guest mode works!"),
        ),
    ))
    .await?;

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let router = Router::new("main")
        .on_guest_message(|observer| observer.register(Handler::new(guest_handler)));

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        .allowed_update(UpdateType::GuestMessage)
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => tracing::info!("Bot stopped"),
        Err(err) => tracing::error!(error = %err, "Bot stopped"),
    }
}
