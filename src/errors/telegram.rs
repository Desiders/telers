use anyhow;
use std::fmt::Debug;
use thiserror;

/// Errors that can be returned by the Telegram Bot API.
/// # Notes
/// This enum isn't complete. If you find a new error, please open an issue or pull request.
/// All possible errors aren't documented in the official Telegram API documentation and usually
/// defined by messages in the responses, but these messages can be changed in the future (frequent situation).
/// So, many errors are represents as [`ErrorKind::BadRequest`], and we are not trying to distinguish them
/// for stability. Thanks Telegram Bot API for this ^_^.
#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("TelegramNetworkError: {message:?}")]
    NetworkError { message: Box<str> },
    #[error(
        "TelegramRetryAfter: {message:?} ({retry_after:?} seconds) (see {url:?} for more info)"
    )]
    RetryAfter {
        url: &'static str, // https://core.telegram.org/bots/faq#my-bot-is-hitting-limits-how-do-i-avoid-this
        message: Box<str>,
        retry_after: i64,
    },
    #[error("TelegramMigrateToChat: {message:?} (migrate to chat id: {migrate_to_chat_id:?}, see {url:?} for more info)")]
    MigrateToChat {
        url: &'static str, // https://core.telegram.org/bots/api#responseparameters
        message: Box<str>,
        migrate_to_chat_id: i64,
    },
    #[error("TelegramBadRequest: {message:?}")]
    BadRequest { message: Box<str> },
    #[error("TelegramNotFound: {message:?}")]
    NotFound { message: Box<str> },
    #[error("TelegramConflictError: {message:?}")]
    ConflictError { message: Box<str> },
    #[error("TelegramForbidden: {message:?}")]
    Forbidden { message: Box<str> },
    #[error("TelegramUnauthorized: {message:?}")]
    Unauthorized { message: Box<str> },
    #[error("TelegramServerError: {message:?}")]
    ServerError { message: Box<str> },
    #[error("TelegramRestartingTelegram: {message:?}")]
    RestartingTelegram { message: Box<str> },
    #[error("TelegramTooLarge: {message:?} (see {url:?} for more info)")]
    EntityTooLarge {
        url: &'static str, // https://core.telegram.org/bots/api#sending-files
        message: Box<str>,
    },
    /// To possible handle unsupported errors, that can be added in the new versions of the Telegram API.
    /// This is necessary to support the "old" API if the "new" API has released a new exception.
    #[error(transparent)]
    UnknownError(#[from] anyhow::Error),
}
