//! This example shows how to receive updates from business connections.
//!
//! You can run this example by setting `BOT_TOKEN` and running:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package business_connection
//! ```

use telers::{
    event::telegram::{Handler, HandlerResult},
    methods::SendMessage,
    types::{BusinessConnection, BusinessMessagesDeleted, Message},
    Bot, Dispatcher, Router,
};

async fn connection(business_connection: BusinessConnection) -> HandlerResult<()> {
    tracing::debug!(?business_connection, "Received business connection");
    Ok(())
}

async fn message(bot: Bot, message: Message) -> HandlerResult<()> {
    tracing::debug!(?message, "Received message");
    bot.send(
        SendMessage::new(message.chat().id(), "Hello world!")
            .business_connection_id(message.business_connection_id().unwrap()),
    )
    .await?;
    Ok(())
}

async fn message_edited(message: Message) -> HandlerResult<()> {
    tracing::debug!(?message, "Received edited message");
    Ok(())
}

async fn messages_deleted(messages_deleted: BusinessMessagesDeleted) -> HandlerResult<()> {
    tracing::debug!(?messages_deleted, "Received deleted messages");
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let mut router = Router::new("main");
    router
        .business_connection
        .register(Handler::new(connection));

    router.business_message.register(Handler::new(message));

    router
        .edited_business_message
        .register(Handler::new(message_edited));

    router
        .deleted_business_messages
        .register(Handler::new(messages_deleted));

    let dispatcher = Dispatcher::builder()
        .allowed_updates(router.resolve_used_update_types())
        .main_router(router.configure_default())
        .bot(bot)
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => tracing::info!("Bot stopped"),
        Err(err) => tracing::error!(error = %err, "Bot stopped"),
    }
}
