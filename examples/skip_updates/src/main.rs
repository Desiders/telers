//! This example shows how to create a bot that skips updates and only processes new updates.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --package skip_updates
//! ```

use telers::{
    enums::UpdateType,
    event::{
        telegram::{Handler, HandlerResult},
        EventReturn,
    },
    methods::DeleteWebhook,
    types::Update,
    Bot, Dispatcher, Router,
};
use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

async fn handler(update: Update) -> HandlerResult {
    event!(Level::INFO, ?update, "Received update");
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
    router.update.register(Handler::new(handler));

    bot.send(DeleteWebhook::new().drop_pending_updates(true))
        .await
        .unwrap();

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        .allowed_updates(UpdateType::all())
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => event!(Level::INFO, "Bot stopped"),
        Err(err) => event!(Level::ERROR, error = %err, "Bot stopped"),
    }
}
