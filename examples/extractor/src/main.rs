//! This example shows how to use [`Extractor`] to extract data and use it in handlers.
//! Check out the documentation of the [`extractor module`] for more information
//!
//! You can run this example by setting `BOT_TOKEN` and running:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package extractor
//! ```
//!
//! [`Extractor`]: telers::Extractor
//! [`FromEvent`]: telers::FromEvent
//! [`FromContext`]: telers::FromContext
//! [`extractor module`]: telers::extractor

use std::convert::Infallible;
use telers::{
    enums::UpdateType,
    errors::{ConvertToTypeError, ExtractionError},
    event::telegram::{Handler, HandlerResult},
    filters::Command,
    methods::SendMessage,
    types::{Message, Update},
    Bot, Context, Dispatcher, Extension, Extractor, FromContext, FromEvent, Request, Router,
};

/// Implementing [`Extractor`] by [`FromEvent`] macros to use struct in handlers.
/// # Notes
/// You can implement it manually, but it's more convenient to use macros to avoid boilerplate code.
#[derive(FromEvent)]
#[event(from = Update)]
struct UpdateId(i64);

impl From<Update> for UpdateId {
    fn from(update: Update) -> Self {
        Self(update.update_id())
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
    // You can use your own error type here
    type Error = ConvertToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        match update.chat() {
            Some(chat) => Ok(Self(chat.id())),
            None => Err(ConvertToTypeError::new("Update", "UpdateChatId")),
        }
    }
}

async fn update_id_handler(
    bot: Bot,
    UpdateId(update_id): UpdateId,
    update_chat_id: Option<UpdateChatId>,
) -> HandlerResult<()> {
    match update_chat_id {
        Some(UpdateChatId(chat_id)) => {
            bot.send(SendMessage::new(chat_id, format!("Update id: {update_id}")))
                .await?;
        }
        None => {
            tracing::warn!("Update doesn't contain chat id");
        }
    }

    Ok(())
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

    async fn extract(request: &Request) -> Result<Self, Self::Error> {
        Ok(Self(
            NumData::extract(request).await?,
            StrData::extract(request).await?,
        ))
    }
}

struct BotId(i64);

/// Implement [`Extractor`] yourself to get bot ID
impl Extractor for BotId {
    type Error = Infallible;

    async fn extract(request: &Request) -> Result<Self, Self::Error> {
        Ok(Self(request.bot.id))
    }
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
) -> HandlerResult<()> {
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
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    // Register handler that sends extracted data to chat
    let router = Router::new("main").on_message(|observer| {
        observer.registers([
            Handler::new(send_data_handler).filter(Command::one("data")),
            Handler::new(update_id_handler).filter(Command::one("update_id")),
        ])
    });

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        .context_extend({
            let mut context = Context::new();
            context.insert("num_data", NumData(1));
            context.insert("str_data", StrData("1"));
            context
        })
        .extension(BoolData(true))
        .allowed_update(UpdateType::Message)
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => tracing::info!("Bot stopped"),
        Err(err) => tracing::error!(error = %err, "Bot stopped"),
    }
}
