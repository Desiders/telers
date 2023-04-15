use telers::{
    client::{Bot, Reqwest, Session},
    dispatcher::{
        event::{telegram::HandlerResult, EventReturn, ToServiceProvider as _},
        Dispatcher, Router,
    },
    enums::UpdateType,
    methods::CopyMessage,
    types::Message,
};

/// This example shows how to use in handler custom client.
/// In this case we use `Reqwest` client, but you can use any other client, which implements `Session` trait.
async fn echo_handler(bot: Bot<impl Session>, message: Message) -> HandlerResult {
    bot.send(
        &CopyMessage::new(message.chat.id, message.chat.id, message.message_id),
        None,
    )
    .await?;

    Ok(EventReturn::Finish)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    pretty_env_logger::init();

    let Ok(bot_token) = std::env::var("BOT_TOKEN") else {
        panic!("BOT_TOKEN env variable is not set!");
    };

    // Create bot with custom client, in this case we use `Reqwest` client, which is default client in `telers`,
    // but you can use any other client, which implements `Session` trait.
    // This call is equivalent to `Bot::new(bot_token)`.
    let bot = Bot::with_client(bot_token, Reqwest::default());

    let mut router = Router::new("main");
    router.message.register_no_filters(echo_handler);

    let dispatcher = Dispatcher::builder()
        .main_router(router)
        .bot(bot)
        .allowed_update(UpdateType::Message)
        .build();

    match dispatcher
        .to_service_provider_default()
        .unwrap()
        .run_polling()
        .await
    {
        Ok(_) => log::info!("Bot stopped"),
        Err(err) => log::error!("Bot stopped with error: {err}"),
    }
}
