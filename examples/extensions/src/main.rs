//! This example shows how to use [`Extensions`] to save data and use it in handlers as [`Extension`].
//! Check out the documentation of the [`extensions module`] for more information.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --package extensions
//! ```
//!
//! [`Extension`]: telers::Extension
//! [`Extensions`]: telers::Extensions
//! [`extensions module`]: telers::extensions

use std::future::Future;
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
    Bot, Dispatcher, Extension, Extensions, Request, Router,
};
use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

#[derive(Debug, Clone, PartialEq, Eq)]
struct NumData(i64);

#[derive(Debug, Clone, PartialEq, Eq)]
struct StrData(&'static str);

#[derive(Clone)]
struct EmptyData;

async fn to_extensions_middleware(
    mut request: Request,
) -> Result<MiddlewareResponse, EventErrorKind> {
    request.extensions.insert(NumData(1));

    Ok((request, EventReturn::default()))
}

fn to_extensions_filter(request: &mut Request) -> impl Future<Output = bool> {
    request.extensions.insert(StrData("1"));
    async move { true }
}

async fn send_data_handler(
    bot: Bot,
    message: Message,
    // Data has been extracted automatically
    Extension(num_data): Extension<NumData>,
    Extension(str_data): Extension<StrData>,
    Extension(_): Extension<EmptyData>,
    // You can use extensions by yourself to extract data
    extensions: Extensions,
) -> HandlerResult {
    assert_eq!(num_data, extensions.get::<NumData>().unwrap().clone());
    assert_eq!(str_data, extensions.get::<StrData>().unwrap().clone());

    bot.send(SendMessage::new(
        message.chat().id(),
        format!("NumData: {:?}. StrData: {:?}", num_data.0, str_data.0),
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

    router
        .message
        .filter(to_extensions_filter)
        // Register handler that sends data from extensions to chat
        .register(Handler::new(send_data_handler).filter(Command::one("data")))
        .outer_middlewares
        // Register middleware that adds data to extensions.
        // Be aware, we register middleware for message observer, so it will be called only for messages.
        // If you want to register middleware for any update, you should register it for update observer.
        .register(to_extensions_middleware);

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        // You also can register an extension using builder methods
        .extension(EmptyData)
        .allowed_update(UpdateType::Message)
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => event!(Level::INFO, "Bot stopped"),
        Err(err) => event!(Level::ERROR, error = %err, "Bot stopped"),
    }
}
