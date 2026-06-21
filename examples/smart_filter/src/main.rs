use telers::{
    enums::{MessageEntityType, UpdateType},
    event::{
        skip_event,
        telegram::{Handler, HandlerResult},
    },
    filters::SmartFilter,
    methods::SendMessage,
    types::{Message, MessageSticker},
    Bot, Dispatcher, Router,
};

async fn uppercase_handler(bot: Bot, message: Message) -> HandlerResult<()> {
    bot.send(SendMessage::new(message.chat().id(), "Uppercase message!"))
        .await?;
    Ok(())
}

async fn lowercase_handler(bot: Bot, message: Message) -> HandlerResult<()> {
    bot.send(SendMessage::new(message.chat().id(), "Lowercase message!"))
        .await?;
    Ok(())
}

async fn has_entities_handler(bot: Bot, message: Message) -> HandlerResult {
    bot.send(SendMessage::new(
        message.chat().id(),
        "Message has bold entities!",
    ))
    .await?;
    // If you want to proceed the next handler. By default propagation finishes after one handler.
    skip_event()
}

async fn long_message_handler(bot: Bot, message: Message) -> HandlerResult {
    bot.send(SendMessage::new(
        message.chat().id(),
        "Long message detected!",
    ))
    .await?;
    // If you want to proceed the next handler. By default propagation finishes after one handler.
    skip_event()
}

async fn sticker_handler(bot: Bot, message: MessageSticker) -> HandlerResult<()> {
    bot.send(SendMessage::new(
        message.chat.id(),
        "Sticker message detected!",
    ))
    .await?;
    Ok(())
}

async fn any_case_handler(bot: Bot, message: Message) -> HandlerResult<()> {
    bot.send(SendMessage::new(message.chat().id(), "Any case message!"))
        .await?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let router = Router::new("main").on_message(|observer| {
        observer.registers([
            Handler::new(long_message_handler).filter(SmartFilter::text().len().gt(100)),
            Handler::new(has_entities_handler).filter(SmartFilter::message().entities().matches(
                |entities| {
                    entities.iter().any(|entity| {
                        matches!(MessageEntityType::from(entity), MessageEntityType::Bold)
                    })
                },
            )),
            Handler::new(sticker_handler).filter(SmartFilter::sticker().is_some()),
            Handler::new(uppercase_handler).filter(SmartFilter::text().is_uppercase()),
            Handler::new(lowercase_handler).filter(SmartFilter::text().is_lowercase()),
            Handler::new(any_case_handler).filter(
                SmartFilter::text()
                    .all()
                    .branch(|val| val.is_uppercase().invert())
                    .branch(|val| val.is_lowercase().invert()),
            ),
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
