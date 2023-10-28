//! This example shows how to create text case filters.
//! First filter checks if the message is uppercase, second filter checks if the message is lowercase.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --package text_case_filters
//! ```

use std::future::Future;

use telers::{
    enums::UpdateType,
    event::{telegram::HandlerResult, EventReturn, ToServiceProvider as _},
    methods::SendMessage,
    types::{Message, Update},
    Bot, Context, Dispatcher, Filter, Router,
};
use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

use async_trait::async_trait;

struct UppercaseFilter;

#[async_trait]
impl<Client> Filter<Client> for UppercaseFilter {
    async fn check(&self, _bot: &Bot<Client>, update: &Update, _context: &Context) -> bool {
        update
            .text()
            .map_or(false, |text| text.to_uppercase() == text)
    }
}

/// # Notes
/// We use here `async move` block to get result without capturing variables
fn lowercase_filter(_bot: &Bot, update: &Update, _context: &Context) -> impl Future<Output = bool>
where {
    let result = update
        .text()
        .map_or(false, |text| text.to_lowercase() == text);

    async move { result }
}

async fn uppercase_handler(bot: Bot, message: Message) -> HandlerResult {
    bot.send(SendMessage::new(message.chat.id, "Uppercase message!"))
        .await?;

    Ok(EventReturn::Finish)
}

async fn lowercase_handler(bot: Bot, message: Message) -> HandlerResult {
    bot.send(SendMessage::new(message.chat.id, "Lowercase message!"))
        .await?;

    Ok(EventReturn::Finish)
}

async fn any_case_handler(bot: Bot, message: Message) -> HandlerResult {
    bot.send(&SendMessage::new(message.chat.id, "Any case message!"))
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
    router
        .message
        .register(uppercase_handler)
        .filter(UppercaseFilter);
    router
        .message
        .register(lowercase_handler)
        .filter(lowercase_filter);
    router.message.register(any_case_handler);

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
