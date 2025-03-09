//! This example shows how to receive updates from business connections.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --package business_connection
//! ```

use telers::{
    event::{telegram::HandlerResult, EventReturn},
    methods::SendMessage,
    types::{BusinessConnection, BusinessMessagesDeleted, Message},
    Bot, Dispatcher, Router,
};
use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

async fn connection(business_connection: BusinessConnection) -> HandlerResult {
    event!(
        Level::DEBUG,
        ?business_connection,
        "Received business connection",
    );

    Ok(EventReturn::Finish)
}

async fn message(bot: Bot, message: Message) -> HandlerResult {
    event!(Level::DEBUG, ?message, "Received message");

    bot.send(
        SendMessage::new(message.chat().id(), "Hello world!")
            .business_connection_id(message.business_connection_id().unwrap()),
    )
    .await?;

    Ok(EventReturn::Finish)
}

async fn message_edited(message: Message) -> HandlerResult {
    event!(Level::DEBUG, ?message, "Received edited message");

    Ok(EventReturn::Finish)
}

async fn messages_deleted(messages_deleted: BusinessMessagesDeleted) -> HandlerResult {
    event!(Level::DEBUG, ?messages_deleted, "Received deleted messages");

    Ok(EventReturn::Finish)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("RUST_LOG"))
        .init();

    let bot = Bot::from_env_by_key("BOT_TOKEN");

    let mut router = Router::new("main");
    router.business_connection.register(connection);
    router.business_message.register(message);
    router.edited_business_message.register(message_edited);
    router.deleted_business_messages.register(messages_deleted);

    let dispatcher = Dispatcher::builder()
        .allowed_updates(router.resolve_used_update_types())
        .main_router(router.configure_default())
        .bot(bot)
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => event!(Level::INFO, "Bot stopped"),
        Err(err) => event!(Level::ERROR, error = %err, "Bot stopped"),
    }
}
