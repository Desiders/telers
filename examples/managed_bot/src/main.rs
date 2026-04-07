//! This example shows how to create a managed bot that echoes all messages it receives,
//! and handles managed bot creation events.
//!
//! The main bot listens for `ManagedBotCreated` messages and `ManagedBot` updates.
//! When a new managed bot is created, it spawns a separate dispatcher for it,
//! sharing the same router configuration and graceful shutdown signal.
//!
//! # Warning
//! This example doesn't show how to save managed bot tokens, it's only for demonstration purposes.
//! It's recommended to use a persistent storage to store managed bot tokens and run them in a separate process.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package managed_bot
//! ```

use std::sync::Arc;
use telers::{
    enums,
    event::telegram::{Handler, HandlerResult},
    filters::MessageType,
    methods::{GetManagedBotToken, SendMessage},
    types::{ManagedBotUpdated, Message, MessageManagedBotCreated},
    utils::shutdown_signal,
    Bot, Dispatcher, DispatcherBuilder, Extension, Router,
};
use tokio::sync::Notify;

async fn managed_bot_created_handler(
    bot: Bot,
    message: MessageManagedBotCreated,
) -> HandlerResult<()> {
    bot.send(SendMessage::new(message.chat.id(), "Managed bot created"))
        .await?;
    Ok(())
}

async fn echo_handler(bot: Bot, message: Message) -> HandlerResult<()> {
    bot.send(message.to_copy_message(message.chat().id()))
        .await?;
    Ok(())
}

async fn managed_bot(
    bot: Bot,
    update: ManagedBotUpdated,
    Extension(builder): Extension<DispatcherBuilder>,
    Extension(shutdown): Extension<Arc<Notify>>,
) -> HandlerResult<()> {
    let token = bot.send(GetManagedBotToken::new(update.bot.id)).await?;
    let bot = Bot::new(token);
    let dispatcher = builder.bot(bot).build();

    // We only spawn the dispatcher in another task.
    // This way, the main dispatcher can continue to listen for updates ,
    // while the new dispatcher handles the managed bot's updates.
    //
    // After main bot is closed, the new dispatcher will be closed as well.
    tokio::spawn(async move {
        dispatcher
            .run_polling()
            .with_graceful_shutdown(async move { shutdown.notified().await })
            .await
            .unwrap();
    });

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let router = Router::new("main")
        .on_message(|observer| {
            observer
                .register(
                    Handler::new(managed_bot_created_handler)
                        .filter(MessageType::one(enums::MessageType::ManagedBotCreated)),
                )
                .register(Handler::new(echo_handler))
        })
        .on_managed_bot(|observer| observer.register(Handler::new(managed_bot)));

    let shutdown = Arc::new(Notify::const_new());

    let builder = Dispatcher::builder()
        .allowed_updates(router.resolve_used_update_types())
        .main_router(router.configure_default());
    let dispatcher = builder
        .clone()
        .extension(builder)
        .extension(shutdown.clone())
        .bot(bot)
        .build();

    let _ = tokio::join!(
        tokio::spawn(run_dispatcher(dispatcher, shutdown.clone())),
        tokio::spawn(handle_shutdown(shutdown))
    );
}

async fn run_dispatcher(dispatcher: Dispatcher, shutdown: Arc<Notify>) {
    dispatcher
        .run_polling()
        .with_graceful_shutdown(async move { shutdown.notified().await })
        .await
        .unwrap();
}

async fn handle_shutdown(shutdown: Arc<Notify>) {
    let () = shutdown_signal().await;
    shutdown.notify_waiters();
}
