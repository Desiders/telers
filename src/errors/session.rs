use super::TelegramErrorKind;

use anyhow;
use thiserror;

/// Possible errors that can occur when processing sending request to the Telegram Bot API and parsing responses from it
/// Usually it's a wrapper for [`TelegramErrorKind`] errors,
/// but it can also been a wrapper for any other error that can occur when sending request or parsing response,
/// for example [`serde_json::Error`].
///
/// Possible Telegram Bot API errors are described in enum [`TelegramErrorKind`], check it out.
#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    /// Error while sending request or decoding response
    #[error(transparent)]
    Client(#[from] anyhow::Error),
    /// Error while parsing JSON
    #[error(transparent)]
    Parse(#[from] serde_json::Error),
    /// Error by Telegram API
    #[error(transparent)]
    Telegram(#[from] TelegramErrorKind),
}
