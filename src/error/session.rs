use super::telegram;

use anyhow;
use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    /// Error while parsing JSON
    #[error(transparent)]
    Parse(#[from] serde_json::Error),
    /// Error by Telegram API
    #[error(transparent)]
    Telegram(#[from] telegram::ErrorKind),
    /// Error while sending request
    #[error(transparent)]
    Request(anyhow::Error),
    /// Error while decoding response
    #[error(transparent)]
    Decode(anyhow::Error),
}
