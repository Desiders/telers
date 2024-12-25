//! This example shows how to use [`Context`] to save data and use it in handlers.
//! Check out the documentation of the [`context module`] for more information, as this example is a small part of its documentation.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --package context
//! ```
//!
//! [`Context`]: telers::Context
//! [`context module`]: telers::context

use async_trait::async_trait;
use telers::{
    enums::UpdateType,
    errors::EventErrorKind,
    event::{telegram::HandlerResult, EventReturn, ToServiceProvider as _},
    filters::Command,
    methods::SendMessage,
    middlewares::{outer::MiddlewareResponse, OuterMiddleware},
    types::Message,
    Bot, Context, Dispatcher, FromContext, Request, Router,
};
use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

// We use `FromContext` here to implement `Extractor` for `Data` which extract it to handler arguments automatically.
// Check `extractor` module for more information.
#[derive(Debug, Clone, PartialEq, Eq, FromContext)]
#[context(key = "data")]
struct Data(i64);

struct ToContextMiddleware<T> {
    data: T,
}

#[async_trait]
impl<T> OuterMiddleware for ToContextMiddleware<T>
where
    T: Send + Sync + Clone + 'static,
{
    async fn call(&self, mut request: Request) -> Result<MiddlewareResponse, EventErrorKind> {
        request.context.insert("data", self.data.clone());

        Ok((request, EventReturn::default()))
    }
}

async fn send_data_handler(
    bot: Bot,
    message: Message,
    // Data has been extracted automatically
    data: Data,
    // You can use context by yourself to extract data
    context: Context,
) -> HandlerResult {
    assert_eq!(data, context.get::<Data>("data").unwrap().clone(),);

    bot.send(SendMessage::new(
        message.chat().id(),
        format!("Data: {data:?}"),
    ))
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

    let data = Data(1);

    // Register middleware that adds data to context.
    // Be aware, we register middleware for message observer, so it will be called only for messages.
    // If you want to register middleware for any update, you should register it for update observer.
    router
        .message
        .outer_middlewares
        .register(ToContextMiddleware { data: data.clone() });
    // Register handler that sends data from context to chat
    router
        .message
        .register(send_data_handler)
        .filter(Command::one("data"));

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
