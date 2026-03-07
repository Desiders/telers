//! This example shows how to serialize Telegram types.
//!
//! You can run this example by setting `BOT_TOKEN` and running:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package serialize
//! ```

use telers::{
    enums::{ParseMode, UpdateType},
    errors::HandlerError,
    event::telegram::{Handler, HandlerResult},
    methods::SendMessage,
    types::Update,
    utils::text::{html_pre_language, html_quote},
    Bot, Dispatcher, Router,
};

async fn serialize_handler(bot: Bot, update: Update) -> HandlerResult<()> {
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
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let router = Router::new("main")
        .on_update(|observer| observer.register(Handler::new(serialize_handler)));

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        .allowed_updates(UpdateType::all())
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => tracing::info!("Bot stopped"),
        Err(err) => tracing::error!(error = %err, "Bot stopped"),
    }
}
