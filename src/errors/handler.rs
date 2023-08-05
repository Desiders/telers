use super::{SessionErrorKind, TelegramErrorKind};

use anyhow;
use thiserror;

/// Error that can occur when processing a handler.
/// Usually it is a wrapper for [`SessionErrorKind`] or [`TelegramErrorKind`] errors,
/// but it can also be a wrapper for any other error, for example, [`std::io::Error`].
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error {
    #[from]
    source: anyhow::Error,
}

impl Error {
    pub fn new(err: impl Into<anyhow::Error>) -> Self {
        Self { source: err.into() }
    }
}

/// To possible to wrap [`TelegramErrorKind`] error in [`Error`] struct without explicit conversion
impl From<TelegramErrorKind> for Error {
    fn from(err: TelegramErrorKind) -> Self {
        Self::new(err)
    }
}

/// To possible to wrap [`SessionErrorKind`] error in [`Error`] struct without explicit conversion
impl From<SessionErrorKind> for Error {
    fn from(err: SessionErrorKind) -> Self {
        Self::new(err)
    }
}
