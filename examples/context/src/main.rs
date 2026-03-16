//! This example shows how to use [`Context`] to save data and use it in handlers.
//! Check out the documentation of the [`context module`] for more information.
//!
//! You can run this example by setting `BOT_TOKEN` and running:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package context
//! ```
//!
//! [`Context`]: telers::Context
//! [`context module`]: telers::context

use telers::{
    enums::UpdateType,
    errors::EventErrorKind,
    event::{
        telegram::{Handler, HandlerResult},
        EventReturn,
    },
    filters::Command,
    methods::SendMessage,
    middlewares::outer::MiddlewareResponse,
    types::Message,
    Bot, Context, Dispatcher, FromContext, Request, Router,
};

// We use `FromContext` here to implement `Extractor` for `Data` which extract it to handler arguments automatically.
// Check `extractor` module for more information.
#[derive(Debug, Clone, PartialEq, Eq, FromContext)]
#[context(key = "data1")]
struct Data1(i64);

#[derive(Debug, Clone, PartialEq, Eq, FromContext)]
#[context(key = "data2")]
struct Data2(i64);

async fn to_context_middleware(mut request: Request) -> Result<MiddlewareResponse, EventErrorKind> {
    request.context.insert("data1", Data1(1));

    Ok((request, EventReturn::default()))
}

async fn send_data_handler(
    bot: Bot,
    message: Message,
    // Data has been extracted automatically
    data1: Data1,
    data2: Data2,
    // You can use context by yourself to extract data
    context: Context,
) -> HandlerResult<()> {
    assert_eq!(data1, context.get::<Data1>("data1").unwrap().clone());
    assert_eq!(data2, context.get::<Data2>("data2").unwrap().clone());

    bot.send(SendMessage::new(
        message.chat().id(),
        format!("Data1: {}. Data2: {}", data1.0, data2.0),
    ))
    .await?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let router = Router::new("main").on_message(|observer| {
        observer
            // Register middleware that adds data to context.
            // Be aware, we register middleware for message observer, so it will be called only for messages.
            // If you want to register middleware for any update, you should register it for update observer.
            .register_outer_middleware(to_context_middleware)
            // Register handler that sends data from context to chat
            .register(Handler::new(send_data_handler).filter(Command::one("data")))
    });

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        // You also can insert data in context using builder methods
        .context("data2", Data2(2))
        .allowed_update(UpdateType::Message)
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => tracing::info!("Bot stopped"),
        Err(err) => tracing::error!(error = %err, "Bot stopped"),
    }
}
