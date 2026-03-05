//! This example shows how to serialize Telegram types.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --package serialize
//! ```

use telers::{
    enums::{ParseMode, UpdateType},
    errors::HandlerError,
    event::{
        telegram::{Handler, HandlerResult},
        EventReturn,
    },
    methods::SendMessage,
    types::Update,
    utils::text::{html_pre_language, html_quote},
    Bot, Dispatcher, Router,
};

use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

async fn serialize_handler(bot: Bot, update: Update) -> HandlerResult {
    if let Some(chat) = update.chat() {
        match serde_json::to_string_pretty(&update) {
            Ok(text) => {
                bot.send(
                    SendMessage::new(chat.id(), html_pre_language(html_quote(text), "json"))
                        .parse_mode(ParseMode::HTML),
                )
                .await?;
            }
            Err(err) => {
                bot.send(SendMessage::new(
                    chat.id(),
                    format!("Serialize error :(\n\n{err:?}"),
                ))
                .await?;

                return Err(HandlerError::new(err));
            }
        }
    }

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
    router.update.register(Handler::new(serialize_handler));

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
