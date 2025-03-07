//! This example shows how to use [`Extractor`] to extract data and use it in handlers.
//! Check out the documentation of the [`extractor module`] for more information
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --package extractor
//! ```
//!
//! [`Extractor`]: telers::Extractor
//! [`FromEvent`]: telers::FromEvent
//! [`FromContext`]: telers::FromContext
//! [`extractor module`]: telers::extractor

use std::convert::Infallible;
use telers::{
    enums::UpdateType,
    errors::{ConvertToTypeError, EventErrorKind, ExtractionError},
    event::{telegram::HandlerResult, EventReturn},
    filters::Command,
    methods::SendMessage,
    middlewares::outer::MiddlewareResponse,
    types::{Message, Update},
    Bot, Dispatcher, Extension, Extractor, FromContext, FromEvent, Request, Router,
};
use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

/// Implementing [`Extractor`] by [`FromEvent`] macros to use struct in handlers.
/// # Notes
/// You can implement it manually, but it's more convenient to use macros to avoid boilerplate code.
#[derive(FromEvent)]
#[event(from = Update)]
struct UpdateId(i64);

impl From<Update> for UpdateId {
    fn from(update: Update) -> Self {
        Self(update.id)
    }
}

/// Implementing [`Extractor`] by [`FromEvent`] macros to use struct in handlers.
/// # Notes
/// You can implement it manually, but it's more convenient to use macros to avoid boilerplate code.
///
/// You can specify custom error type by `[event(error = ...)]`, default it's [`telers::errors::ConvertToTypeError`].
/// The error type in macros should be the same as in the implementation of [`TryFrom`].
///
/// The trait also is implemented for `Option<T>`, `Result<T, E>` where `T: Extractor`,
/// so we can use `Option<UpdateChatId>` in handlers (or `Result<UpdateChatId, ConvertToTypeError>`).
#[derive(FromEvent)]
#[event(try_from = Update)]
struct UpdateChatId(i64);

impl TryFrom<Update> for UpdateChatId {
    type Error = ConvertToTypeError; // You can use your own error type here

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        match update.chat_id() {
            Some(chat_id) => Ok(Self(chat_id)),
            None => Err(ConvertToTypeError::new("Update", "UpdateChatId")),
        }
    }
}

async fn update_id_handler(
    bot: Bot,
    UpdateId(update_id): UpdateId,
    update_chat_id: Option<UpdateChatId>,
) -> HandlerResult {
    match update_chat_id {
        Some(UpdateChatId(chat_id)) => {
            bot.send(SendMessage::new(chat_id, format!("Update id: {update_id}")))
                .await?;
        }
        None => {
            event!(Level::WARN, "Update doesn't contain chat id");
        }
    }

    Ok(EventReturn::Finish)
}

#[derive(Debug, Clone, PartialEq, Eq, FromContext)]
#[context(key = "num_data")]
struct NumData(i64);

#[derive(Debug, Clone, PartialEq, Eq, FromContext)]
#[context(key = "str_data")]
struct StrData(&'static str);

#[derive(Clone)]
struct BoolData(bool);

#[derive(Clone)]
struct DataCombined(NumData, StrData);

/// Implement [`Extractor`] yourself and just use what is implemented automatically for [`NumData`] and [`StrData`].
impl Extractor for DataCombined {
    type Error = ExtractionError;

    fn extract(request: &Request) -> Result<Self, Self::Error> {
        Ok(Self(NumData::extract(request)?, StrData::extract(request)?))
    }
}

struct BotId(i64);

/// Implement [`Extractor`] yourself to get bot ID
impl Extractor for BotId {
    type Error = Infallible;

    fn extract(request: &Request) -> Result<Self, Self::Error> {
        Ok(Self(request.bot.id))
    }
}

async fn to_context_and_extensions(
    mut request: Request,
) -> Result<MiddlewareResponse, EventErrorKind> {
    request.context.insert("num_data", NumData(1));
    request.context.insert("str_data", StrData("1"));

    request.extensions.insert(BoolData(true));

    Ok((request, EventReturn::default()))
}

async fn send_data_handler(
    bot: Bot,
    message: Message,
    num_data1: NumData,
    str_data1: StrData,
    // This structure is created by extractor that we implemented
    DataCombined(num_data2, str_data2): DataCombined,
    // This structure is created by middleware, we haven't implemented any trait for it,
    // but we can still use it because extractor is implemented for all extensions
    Extension(BoolData(bool_data)): Extension<BoolData>,
    BotId(bot_id): BotId,
) -> HandlerResult {
    assert_eq!(num_data1, num_data2);
    assert_eq!(str_data1, str_data2);

    bot.send(SendMessage::new(
        message.chat().id(),
        format!(
            "NumData: {:?}. StrData: {:?}. BoolData: {bool_data}. BotId: {bot_id}",
            num_data1.0, str_data1.0
        ),
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

    // Register middleware that adds data to context and extensions.
    // Be aware, we register middleware for message observer, so it will be called only for messages.
    // If you want to register middleware for any update, you should register it for update observer.
    router
        .message
        .outer_middlewares
        .register(to_context_and_extensions);
    // Register handler that sends extracted data to chat
    router
        .message
        .register(send_data_handler)
        .filter(Command::one("data"));
    router
        .message
        .register(update_id_handler)
        .filter(Command::one("update_id"));

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
