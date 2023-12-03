//! This example shows how to format text sending by the bot.
//! In this example we use HTML formatting, but you can use Markdown formatting too, but you can't use both formatting in one message.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --package echo_bot
//! ```

use telers::{
    enums::{ParseMode, UpdateType},
    event::{telegram::HandlerResult, EventReturn, ToServiceProvider as _},
    methods::SendMessage,
    types::Message,
    utils::text::{html_text_link, Builder as TextBuilder, Formatter as _, HTMLFormatter},
    Bot, Dispatcher, Router,
};
use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

async fn handler(bot: Bot, message: Message) -> HandlerResult {
    // First way to format text by using formatting directly in the text.
    let text = "This is <b>bold</b> text.\nThis is <i>italic</i> text.\nThis is <a href=\"https://example.com\">link</a>.";

    // We should use `parse_mode` to specify that we use HTML formatting.
    bot.send(SendMessage::new(message.chat().id(), text).parse_mode(ParseMode::HTML))
        .await?;

    // Second way to format text by using `TextBuilder`.
    let text_builder = TextBuilder::new(HTMLFormatter::default())
        .text("This is ")
        .bold("bold")
        .text(" text.\nThis is ")
        .italic("italic")
        .text(" text.\nThis is ")
        .text_link("link", "https://example.com")
        .text(".");
    let text = text_builder.get_text();

    bot.send(SendMessage::new(message.chat().id(), text).parse_mode(ParseMode::HTML))
        .await?;

    // Third way to format text by using `HTMLFormatter`.
    let html = HTMLFormatter::default();

    let text = format!(
        "This is {bold} text.\nThis is {italic} text.\nThis is {link}.",
        bold = html.bold("bold"),
        italic = html.italic("italic"),
        // We also can use short functions to format text. They are the same as methods of `HTMLFormatter`
        link = html_text_link("link", "https://example.com")
    );

    bot.send(SendMessage::new(message.chat().id(), text).parse_mode(ParseMode::HTML))
        .await?;

    Ok(EventReturn::Finish)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("RUST_LOG"))
        .init();

    let Ok(bot_token) = std::env::var("BOT_TOKEN") else {
        panic!("BOT_TOKEN env variable is not set!");
    };

    let bot = Bot::new(bot_token);

    let mut router = Router::new("main");
    router.message.register(handler);

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
        Ok(()) => event!(Level::INFO, "Bot stopped"),
        Err(err) => event!(Level::ERROR, error = %err, "Bot stopped"),
    }
}
