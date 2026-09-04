//! This module contains the [`Error`] struct, which is a wrapper for any error that can occur when processing a handler.
//! We use this wrapper around [`anyhow::Error`], because it allows us to wrap any error type, including custom errors
//! and don't use [`anyhow::Error`] directly.
//!
//! Usually it is a wrapper for [`SessionErrorKind`] or [`TelegramErrorKind`] errors,
//! but it can also be a wrapper for any another error.

use super::{DownloadErrorKind, SessionErrorKind, TelegramErrorKind};

use anyhow;
use std::sync::Arc;

/// A wrapper for any error that can occur when processing a handler.
/// We use this wrapper around [`anyhow::Error`], because it allows us to wrap any error type, including custom errors
/// and don't use [`anyhow::Error`] directly.
/// Usually it is a wrapper for [`SessionErrorKind`] or [`TelegramErrorKind`] errors,
/// but it can also be a wrapper for any another error.
///
/// The source error is wrapped in [`Arc`], so the [`Error`] can be cloned cheaply
/// and shared (for example, with error handlers).
#[derive(Clone, Debug)]
pub struct Error {
    source: Arc<anyhow::Error>,
}

impl Error {
    /// # Arguments
    /// * `err` - The error to wrap.
    /// # Notes
    /// If you want to pass just a message, you can use [`Error::from_display`] or [`Error::from_debug`] methods.
    pub fn new(err: impl Into<anyhow::Error>) -> Self {
        Self {
            source: Arc::new(err.into()),
        }
    }

    /// # Arguments
    /// * `info` - The error message.
    /// # Notes
    /// This method is useful when you want to pass just a message.
    /// If you want to pass an error, you can use [`Error::new`] method.
    pub fn from_display(info: impl std::fmt::Display) -> Self {
        Self::new(anyhow::anyhow!("{info}"))
    }

    /// # Arguments
    /// * `info` - The error message.
    /// # Notes
    /// This method is useful when you want to pass just a message.
    /// If you want to pass an error, you can use [`Error::new`] method.
    pub fn from_debug(info: impl std::fmt::Debug) -> Self {
        Self::new(anyhow::anyhow!("{info:?}"))
    }

    /// Returns a reference to the source error.
    /// Useful for downcasting to a concrete error type (e.g. `err.as_anyhow().downcast_ref::<MyError>()`).
    ///
    /// # Notes
    /// Named `as_anyhow` (not `source`) to avoid shadowing [`std::error::Error::source`],
    /// which would silently change the meaning of `err.source()` calls.
    #[must_use]
    pub fn as_anyhow(&self) -> &anyhow::Error {
        &self.source
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&*self.source, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().source()
    }
}

/// To possible to wrap [`TelegramErrorKind`] error in [`Error`] struct without boilerplate code
impl From<TelegramErrorKind> for Error {
    fn from(err: TelegramErrorKind) -> Self {
        Self::new(err)
    }
}

/// To possible to wrap [`SessionErrorKind`] error in [`Error`] struct without boilerplate code
impl From<SessionErrorKind> for Error {
    fn from(err: SessionErrorKind) -> Self {
        Self::new(err)
    }
}

/// To possible to wrap [`DownloadErrorKind`] error in [`Error`] struct without boilerplate code
impl From<DownloadErrorKind> for Error {
    fn from(err: DownloadErrorKind) -> Self {
        Self::new(err)
    }
}

/// To possible to wrap [`std::convert::Infallible`] error in [`Error`] struct without boilerplate code
impl From<std::convert::Infallible> for Error {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!("Infallible error type should never be constructed")
    }
}

/// To possible to wrap [`std::io::Error`] error in [`Error`] struct without boilerplate code
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::new(err)
    }
}

/// To possible to wrap [`std::fmt::Error`] error in [`Error`] struct without boilerplate code
impl From<std::fmt::Error> for Error {
    fn from(err: std::fmt::Error) -> Self {
        Self::new(err)
    }
}

/// To possible to wrap [`std::num::ParseIntError`] error in [`Error`] struct without boilerplate code
impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::new(err)
    }
}

/// To possible to wrap [`std::num::ParseFloatError`] error in [`Error`] struct without boilerplate code
impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Self {
        Self::new(err)
    }
}

impl<T> From<Box<T>> for Error
where
    T: std::error::Error + Send + Sync + 'static,
{
    fn from(err: Box<T>) -> Self {
        Self::new(err)
    }
}
