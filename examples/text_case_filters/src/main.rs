//! This example shows how to create text case filters.
//! First filter checks if the message is uppercase, second filter checks if the message is lowercase.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --package text_case_filters
//! ```

use async_trait::async_trait;
use std::future::Future;
use telers::{
    enums::UpdateType,
    event::{telegram::HandlerResult, EventReturn},
    methods::SendMessage,
    types::Message,
    Bot, Dispatcher, Filter, Request, Router,
};
use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

#[derive(Clone)]
struct UppercaseFilter;

#[async_trait]
impl Filter for UppercaseFilter {
    async fn check(&mut self, request: &mut Request) -> bool {
        request
            .update
            .text()
            .is_some_and(|text| text.to_uppercase() == text)
    }
}

fn lowercase_filter(request: &mut Request) -> impl Future<Output = bool> {
    let result = request
        .update
        .text()
        .is_some_and(|text| text.to_lowercase() == text);

    async move { result }
}

async fn uppercase_handler(bot: Bot, message: Message) -> HandlerResult {
    bot.send(SendMessage::new(message.chat().id(), "Uppercase message!"))
        .await?;

    Ok(EventReturn::Finish)
}

async fn lowercase_handler(bot: Bot, message: Message) -> HandlerResult {
    bot.send(SendMessage::new(message.chat().id(), "Lowercase message!"))
        .await?;

    Ok(EventReturn::Finish)
}

async fn any_case_handler(bot: Bot, message: Message) -> HandlerResult {
    bot.send(SendMessage::new(message.chat().id(), "Any case message!"))
        .await?;

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
        .main_router(router.configure_default())
        .bot(bot)
        .allowed_update(UpdateType::Message)
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => event!(Level::INFO, "Bot stopped"),
        Err(err) => event!(Level::ERROR, error = %err, "Bot stopped"),
    }
}
