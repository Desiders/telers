use super::TelegramErrorKind;

use anyhow;
use thiserror;

/// This enum represents all possible errors that can occur in the process of sending requests to the Telegram API
/// and parsing responses
#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    /// Error while sending request or decoding response
    #[error(transparent)]
    Client(anyhow::Error),
    /// Error while parsing JSON
    #[error(transparent)]
    Parse(#[from] serde_json::Error),
    /// Error by Telegram API
    #[error(transparent)]
    Telegram(#[from] TelegramErrorKind),
}
