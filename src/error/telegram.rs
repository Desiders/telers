use anyhow;
use std::fmt::Debug;
use thiserror;

/// This enum represents all possible errors that can be returned from Telegram API
/// # Note
/// This enum isn't complete. If you find a new error, please open an issue or pull request.
///
/// All possible errors aren't documented in the official Telegram API documentation and usually
/// defined by messages in the responses, but these messages can be changed in the future (frequent situation).
/// So, many errors are represents as [`ErrorKind::BadRequest`], and we are not trying to distinguish them
/// for stability. Thanks Telegram API for this ^_^
#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("TelegramNetworkError: {message:?}")]
    NetworkError { message: String },
    #[error(
        "TelegramRetryAfter: {message:?} ({retry_after:?} seconds) (see {url:?} for more info)"
    )]
    RetryAfter {
        url: &'static str, // https://core.telegram.org/bots/faq#my-bot-is-hitting-limits-how-do-i-avoid-this
        message: String,
        retry_after: i64,
    },
    #[error("TelegramMigrateToChat: {message:?} (migrate to chat id: {migrate_to_chat_id:?}, see {url:?} for more info)")]
    MigrateToChat {
        url: &'static str, // https://core.telegram.org/bots/api#responseparameters
        message: String,
        migrate_to_chat_id: i64,
    },
    #[error("TelegramBadRequest: {message:?}")]
    BadRequest { message: String },
    #[error("TelegramNotFound: {message:?}")]
    NotFound { message: String },
    #[error("TelegramConflictError: {message:?}")]
    ConflictError { message: String },
    #[error("TelegramForbidden: {message:?}")]
    Forbidden { message: String },
    #[error("TelegramUnauthorized: {message:?}")]
    Unauthorized { message: String },
    #[error("TelegramServerError: {message:?}")]
    ServerError { message: String },
    #[error("TelegramRestartingTelegram: {message:?}")]
    RestartingTelegram { message: String },
    #[error("TelegramTooLarge: {message:?} (see {url:?} for more info)")]
    EntityTooLarge {
        url: &'static str, // https://core.telegram.org/bots/api#sending-files
        message: String,
    },
    /// To possible handle unsupported errors, that can be added in the new versions of the Telegram API.
    /// This is necessary to support the "old" API if the "new" API has released a new exception.
    #[error(transparent)]
    UnknownError(#[from] anyhow::Error),
}
