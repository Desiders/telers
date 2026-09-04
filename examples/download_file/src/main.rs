use telers::{
    enums::UpdateType,
    event::telegram::{Handler, HandlerResult},
    methods::SendMessage,
    types::{Message, MessageDocument, MessagePhoto},
    Bot, Dispatcher, Router,
};

async fn photo_handler(bot: Bot, message: MessagePhoto) -> HandlerResult<()> {
    let photo = message.photo.last().unwrap();

    let bytes = bot.download(photo).await?.bytes().await?;

    bot.send(SendMessage::new(
        message.chat.id(),
        format!("Downloaded photo of {} bytes", bytes.len()),
    ))
    .await?;

    Ok(())
}

async fn document_handler(bot: Bot, message: MessageDocument) -> HandlerResult<()> {
    let document = &message.document;
    let file_name = document
        .file_name
        .as_deref()
        .unwrap_or(&document.file_unique_id);

    bot.download(document).await?.to_path(file_name).await?;

    bot.send(SendMessage::new(
        message.chat.id(),
        format!("Saved document as `{file_name}`"),
    ))
    .await?;

    Ok(())
}

async fn fallback_handler(bot: Bot, message: Message) -> HandlerResult<()> {
    bot.send(SendMessage::new(
        message.chat().id(),
        "Send me a photo or a document to download it",
    ))
    .await?;

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let router = Router::new("main").on_message(|observer| {
        observer
            .register(Handler::new(photo_handler))
            .register(Handler::new(document_handler))
            .register(Handler::new(fallback_handler))
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
