//! This example shows how to create text case filters.
//! First filter checks if the message is uppercase, second filter checks if the message is lowercase.
//!
//! You can run this example by setting `BOT_TOKEN` and running:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package text_case_filters
//! ```

use std::{convert::Infallible, future::Future};
use telers::{
    enums::UpdateType,
    event::telegram::{Handler, HandlerResult},
    methods::SendMessage,
    types::MessageText,
    Bot, Dispatcher, Filter, Request, Router,
};

#[derive(Clone)]
struct UppercaseFilter;

impl Filter for UppercaseFilter {
    type Error = Infallible;

    async fn check(&mut self, request: &mut Request) -> Result<bool, Infallible> {
        Ok(request
            .update
            .text()
            .is_some_and(|text| text.to_uppercase() == text))
    }
}

fn lowercase_filter(request: &mut Request) -> impl Future<Output = Result<bool, Infallible>> {
    let result = request
        .update
        .text()
        .is_some_and(|text| text.to_lowercase() == text);
    async move { Ok(result) }
}

async fn uppercase_handler(bot: Bot, message: MessageText) -> HandlerResult<()> {
    bot.send(SendMessage::new(message.chat.id(), "Uppercase message!"))
        .await?;
    Ok(())
}

async fn lowercase_handler(bot: Bot, message: MessageText) -> HandlerResult<()> {
    bot.send(SendMessage::new(message.chat.id(), "Lowercase message!"))
        .await?;
    Ok(())
}

async fn any_case_handler(bot: Bot, message: MessageText) -> HandlerResult<()> {
    bot.send(SendMessage::new(message.chat.id(), "Any case message!"))
        .await?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let mut router = Router::new("main");
    router.message.registers([
        Handler::new(uppercase_handler).filter(UppercaseFilter),
        Handler::new(lowercase_handler).filter(lowercase_filter),
        Handler::new(any_case_handler),
    ]);

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
