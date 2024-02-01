//! This example shows how to use [`FromEventAndContext`] to extract data from event and context and use it in handlers.
//! Check out the documentation of the [`extractors`] module for more information, as this example is a small part of its documentation.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --package from_event_and_context
//! ```
//!
//! [`FromEventAndContext`]: telers::extractors::FromEventAndContext
//! [`FromEvent`]: telers::extractors::FromEvent
//! [`FromContext`]: telers::extractors::FromContext
//! [`extractors`]: telers::extractors

use async_trait::async_trait;
use telers::{
    enums::UpdateType,
    errors::{ConvertToTypeError, EventErrorKind},
    event::{telegram::HandlerResult, EventReturn, ToServiceProvider as _},
    extractors::FromEvent,
    filters::Command,
    methods::SendMessage,
    middlewares::{outer::MiddlewareResponse, OuterMiddleware},
    router::Request as RouterRequest,
    types::{Message, Update},
    Bot, Dispatcher, FromContext, Router,
};
use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

/// Implementing [`telers::extractors::FromEventAndContext`] by [`FromEvent`] macros to use struct in handlers.
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

/// Implementing [`telers::extractors::FromEventAndContext`] by [`FromEvent`] macros to use struct in handlers.
/// # Notes
/// You can implement it manually, but it's more convenient to use macros to avoid boilerplate code.
///
/// You can specify custom error type by `[event(error = ...)]`, default it's `telers::errors::ConvertToTypeError`.
/// The error type in macros should be the same as in the implementation of [`TryFrom`].
///
/// The trait also is implemented for `Option<T>`, `Result<T, E>` where `T: FromEventAndContext`,
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

/// Handler that sends update id to chat from which we get the update.
/// # Arguments
/// * `bot` - Bot instance
/// * `update_id` - Update id that we get from [`Update`] by extractor
/// * `update_chat_id` - Update chat id that we get from [`Update`] by extractor
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

/// # Warning
/// Be aware that you should use [`Clone`] trait for data that you want to add to context.
#[derive(Debug, Clone, FromContext)]
#[context(key = "data")]
struct Data(i64);

/// Middleware that adds data to context.
/// # Arguments
/// * `key` - Key for data in context
/// * `data` - Data that we want to add to context
struct ToContextMiddleware<T> {
    key: &'static str,
    data: T,
}

#[async_trait]
impl<T> OuterMiddleware for ToContextMiddleware<T>
where
    T: Send + Sync + Clone + 'static,
{
    async fn call(&self, request: RouterRequest) -> Result<MiddlewareResponse, EventErrorKind> {
        request
            .context
            .insert(self.key, Box::new(self.data.clone()));

        Ok((request, EventReturn::default()))
    }
}

/// Handler that sends data from context to chat.
/// # Arguments
/// * `bot` - Bot instance
/// * `message` - Message instance
/// * `data` - Data that we get from context by middleware
async fn send_data_handler(bot: Bot, message: Message, data: Data) -> HandlerResult {
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

    let Ok(bot_token) = std::env::var("BOT_TOKEN") else {
        panic!("BOT_TOKEN env variable is not set!");
    };

    let bot = Bot::new(bot_token);

    let mut router = Router::new("main");

    // Register middleware that adds data to context
    router
        .update
        .outer_middlewares
        .register(ToContextMiddleware {
            key: "data",
            data: Data(1),
        });
    // Register handler that sends data from context to chat
    router
        .message
        .register(send_data_handler)
        .filter(Command::one("data"));

    // We register handler for all events just for example, it doesn't make sense
    router.update.register(update_id_handler);

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
