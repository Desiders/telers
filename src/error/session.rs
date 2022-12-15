use super::telegram;

use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error(transparent)]
    Parse(#[from] serde_json::Error),
    #[error(transparent)]
    Telegram(#[from] telegram::ErrorKind),
}
