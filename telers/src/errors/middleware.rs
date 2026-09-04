//! This module contains the [`Error`] struct, which is a wrapper for any error that can occur when processing a middleware.
//! We use this wrapper around [`anyhow::Error`], because it allows us to wrap any error type, including custom errors
//! and don't use [`anyhow::Error`] directly.
//!
//! Usually it is a wrapper for [`HandlerError`] errors, but it can also be a wrapper for any another error.
//!
//! [`HandlerError`]: telers::errors::HandlerError

use anyhow;
use std::{
    convert::Infallible,
    fmt, io,
    num::{ParseFloatError, ParseIntError},
    sync::Arc,
};

/// A wrapper for any error that can occur when processing a middleware.
/// We use this wrapper around [`anyhow::Error`], because it allows us to wrap any error type, including custom errors
/// and don't use [`anyhow::Error`] directly.
/// Usually it is a wrapper for [`crate::errors::HandlerError`] errors, but it can also be a wrapper for any another error.
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
    pub fn from_display(info: impl fmt::Display) -> Self {
        Self::new(anyhow::anyhow!("{info}"))
    }

    /// # Arguments
    /// * `info` - The error message.
    /// # Notes
    /// This method is useful when you want to pass just a message.
    /// If you want to pass an error, you can use [`Error::new`] method.
    pub fn from_debug(info: impl fmt::Debug) -> Self {
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&*self.source, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().source()
    }
}

/// To possible to wrap [`super::TelegramErrorKind`] error in [`Error`] struct without boilerplate code
impl From<super::TelegramErrorKind> for Error {
    fn from(err: super::TelegramErrorKind) -> Self {
        Self::new(err)
    }
}

/// To possible to wrap [`super::SessionErrorKind`] error in [`Error`] struct without boilerplate code
impl From<super::SessionErrorKind> for Error {
    fn from(err: super::SessionErrorKind) -> Self {
        Self::new(err)
    }
}

/// To possible to wrap [`Infallible`] error in [`Error`] struct without boilerplate code
impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!("Infallible error type should never be constructed")
    }
}

/// To possible to wrap [`io::Error`] error in [`Error`] struct without boilerplate code
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::new(err)
    }
}

/// To possible to wrap [`fmt::Error`] error in [`Error`] struct without boilerplate code
impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Self::new(err)
    }
}

/// To possible to wrap [`ParseIntError`] error in [`Error`] struct without boilerplate code
impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Self::new(err)
    }
}

/// To possible to wrap [`ParseFloatError`] error in [`Error`] struct without boilerplate code
impl From<ParseFloatError> for Error {
    fn from(err: ParseFloatError) -> Self {
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
