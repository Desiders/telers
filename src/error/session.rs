use super::telegram;

use anyhow;
use thiserror;

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
    Telegram(#[from] telegram::ErrorKind),
}
