//! This module contains the error type that can occur when extracting arguments from an event and a context to pass to a handler.
//! For example, if you try to extract [`User`] from the [`Context`] and the context does not contain the user, you can use this error type.
//!
//! This error type used usually in [`FromEventAndContext`] trait implementations when the extraction fails.
//!
//! [`FromEventAndContext`]: crate::extractors::FromEventAndContext
//! [`User`]: crate::types::User
//! [`Context`]: crate::Context

use std::{borrow::Cow, convert::Infallible};
use thiserror;

/// An error that can occur when extracting arguments from an event and a context to pass to a handler.
/// For example, if you try to extract [`crate::types::User`] from the [`crate::Context`] and the context does not contain the user,
/// you can use this error type.
#[derive(Debug, thiserror::Error)]
#[error("Extraction error: {msg}")]
pub struct Error {
    msg: Cow<'static, str>,
}

impl Error {
    pub fn new(msg: impl Into<Cow<'static, str>>) -> Self {
        Self { msg: msg.into() }
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!("Infallible error type should never be constructed")
    }
}
