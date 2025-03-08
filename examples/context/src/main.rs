//! This example shows how to use [`Context`] to save data and use it in handlers.
//! Check out the documentation of the [`context module`] for more information.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --package context
//! ```
//!
//! [`Context`]: telers::Context
//! [`context module`]: telers::context

use telers::{
    enums::UpdateType,
    errors::EventErrorKind,
    event::{telegram::HandlerResult, EventReturn},
    filters::Command,
    methods::SendMessage,
    middlewares::outer::MiddlewareResponse,
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

async fn to_context_middleware(mut request: Request) -> Result<MiddlewareResponse, EventErrorKind> {
    request.context.insert("data", Data(1));

    Ok((request, EventReturn::default()))
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
        format!("Data: {}", data.0),
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

    // Register middleware that adds data to context.
    // Be aware, we register middleware for message observer, so it will be called only for messages.
    // If you want to register middleware for any update, you should register it for update observer.
    router
        .message
        .outer_middlewares
        .register(to_context_middleware);
    // Register handler that sends data from context to chat
    router
        .message
        .register(send_data_handler)
        .filter(Command::one("data"));

    let mut dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        .allowed_update(UpdateType::Message)
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => event!(Level::INFO, "Bot stopped"),
        Err(err) => event!(Level::ERROR, error = %err, "Bot stopped"),
    }
}
