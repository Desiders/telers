use std::{convert::Infallible, future::Future};
use telers::{
    enums::UpdateType,
    event::telegram::{Handler, HandlerResult},
    methods::SendMessage,
    types::Message,
    Bot, Dispatcher, Filter, FilterResult, Request, Router,
};

#[derive(Clone)]
struct UppercaseFilter;

impl Filter for UppercaseFilter {
    type Error = Infallible;

    async fn check(&mut self, request: &mut Request) -> FilterResult<Infallible> {
        Ok(request
            .update
            .text()
            .is_some_and(|text| text.to_uppercase() == text))
    }
}

fn lowercase_filter(request: &mut Request) -> impl Future<Output = FilterResult<Infallible>> {
    let result = request
        .update
        .text()
        .is_some_and(|text| text.to_lowercase() == text);
    async move { Ok(result) }
}

async fn uppercase_handler(bot: Bot, message: Message) -> HandlerResult<()> {
    bot.send(SendMessage::new(message.chat().id(), "Uppercase message!"))
        .await?;
    Ok(())
}

async fn lowercase_handler(bot: Bot, message: Message) -> HandlerResult<()> {
    bot.send(SendMessage::new(message.chat().id(), "Lowercase message!"))
        .await?;
    Ok(())
}

async fn any_case_handler(bot: Bot, message: Message) -> HandlerResult<()> {
    bot.send(SendMessage::new(message.chat().id(), "Any case message!"))
        .await?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let router = Router::new("main").on_message(|observer| {
        observer.registers([
            Handler::new(uppercase_handler).filter(UppercaseFilter),
            Handler::new(lowercase_handler).filter(lowercase_filter),
            Handler::new(any_case_handler)
                .filter(UppercaseFilter.invert())
                .filter(lowercase_filter.invert()),
        ])
    });

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
