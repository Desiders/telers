//! This module contains the [`Error`] struct, which is a wrapper for any error that can occur when processing a middleware.
//! We use this wrapper around [`anyhow::Error`], because it allows us to wrap any error type, including custom errors
//! and don't use [`anyhow::Error`] directly.
//!
//! Usually it is a wrapper for [`HandlerError`] errors, but it can also be a wrapper for any another error.
//!
//! [`HandlerError`]: crate::errors::HandlerError

use anyhow;
use thiserror;

/// A wrapper for any error that can occur when processing a middleware.
/// We use this wrapper around [`anyhow::Error`], because it allows us to wrap any error type, including custom errors
/// and don't use [`anyhow::Error`] directly.
/// Usually it is a wrapper for [`crate::errors::HandlerError`] errors, but it can also be a wrapper for any another error.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error {
    #[from]
    source: anyhow::Error,
}

impl Error {
    /// # Arguments
    /// * `err` - The error to wrap.
    /// # Notes
    /// If you want to pass just a message, you can use [`Error::from_display`] or [`Error::from_debug`] methods.
    pub fn new(err: impl Into<anyhow::Error>) -> Self {
        Self { source: err.into() }
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
