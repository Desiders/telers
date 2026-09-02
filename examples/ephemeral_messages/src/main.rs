use telers::{
    enums::UpdateType,
    event::telegram::{Handler, HandlerResult},
    filters::Command,
    methods::{DeleteEphemeralMessage, EditEphemeralMessageText, SendMessage},
    types::{EphemeralMessageParameters, Message},
    Bot, Dispatcher, Router,
};

async fn whisper_handler(bot: Bot, message: Message) -> HandlerResult<()> {
    let Some(user) = message.from() else {
        return Ok(());
    };

    let sent = bot
        .send(
            SendMessage::new(
                message.chat().id(),
                format!(
                    "Hi {}! Only you can see this message in the group.",
                    user.first_name
                ),
            )
            .ephemeral_message_parameters(EphemeralMessageParameters::new(user.id)),
        )
        .await?;
    let ephemeral_message_id = sent.ephemeral_message_id().unwrap();

    bot.send(
        EditEphemeralMessageText::new(message.chat().id(), user.id, ephemeral_message_id)
            .text("(edited) This whisper is still visible only to you."),
    )
    .await?;

    Ok(())
}

async fn cleanup_handler(bot: Bot, message: Message) -> HandlerResult<()> {
    let Some(user) = message.from() else {
        return Ok(());
    };

    let sent = bot
        .send(
            SendMessage::new(message.chat().id(), "This whisper will self-destruct...")
                .ephemeral_message_parameters(EphemeralMessageParameters::new(user.id)),
        )
        .await?;
    let ephemeral_message_id = sent.ephemeral_message_id().unwrap();

    bot.send(DeleteEphemeralMessage::new(
        message.chat().id(),
        user.id,
        ephemeral_message_id,
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
            .register(Handler::new(whisper_handler).filter(Command::one("whisper")))
            .register(Handler::new(cleanup_handler).filter(Command::one("cleanup")))
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
