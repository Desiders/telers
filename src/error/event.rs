use super::{SessionErrorKind, TelegramErrorKind};

use anyhow;
use thiserror;

/// Error, which can be returned from handlers, filters and middlewares by user
#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct Error {
    #[from]
    source: anyhow::Error,
}

impl Error {
    pub fn new<T: Into<anyhow::Error>>(err: T) -> Self {
        Self { source: err.into() }
    }
}

impl From<TelegramErrorKind> for Error {
    fn from(err: TelegramErrorKind) -> Self {
        Self::new(err)
    }
}

impl From<SessionErrorKind> for Error {
    fn from(err: SessionErrorKind) -> Self {
        Self::new(err)
    }
}
