use super::{SessionErrorKind, TelegramErrorKind};

use anyhow;
use thiserror;

/// This struct represents all possible errors that can occur in the handlers
///
/// Usually, in handlers returns [`SessionErrorKind`] error or [`TelegramErrorKind`] error,
/// but you can return any error that implements [`Into<anyhow::Error>`] trait.
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

/// To possible to wrap [`TelegramErrorKind`] errors in [`Error`] struct without explicit conversion
impl From<TelegramErrorKind> for Error {
    fn from(err: TelegramErrorKind) -> Self {
        Self::new(err)
    }
}

/// To possible to wrap [`SessionErrorKind`] errors in [`Error`] struct without explicit conversion
impl From<SessionErrorKind> for Error {
    fn from(err: SessionErrorKind) -> Self {
        Self::new(err)
    }
}
