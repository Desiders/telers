//! This module contains the error type that can occur when extracting arguments from an event and a context to pass to a handler.
//! For example, if you try to extract [`User`] from the [`Context`] and the context does not contain the user, you can use this error type.
//!
//! This error type used usually in [`Extractor`] trait implementations when the extraction fails.
//!
//! [`Extractor`]: telers::Extractor
//! [`User`]: telers::types::User
//! [`Context`]: telers::Context

use anyhow;
use std::{
    borrow::Cow,
    convert::Infallible,
    fmt::{Debug, Display},
    sync::Arc,
};
use thiserror;

/// An error that can occur when extracting arguments from an event and a context to pass to a handler.
/// For example, if you try to extract [`crate::types::User`] from the [`crate::Context`] and the context does not contain the user,
/// you can use this error type.
#[derive(Debug, Clone, thiserror::Error)]
#[error("Extraction error: {msg}")]
pub struct Error {
    msg: Cow<'static, str>,
    source: Option<Arc<anyhow::Error>>,
}

impl Error {
    pub fn new(msg: impl Into<Cow<'static, str>>) -> Self {
        Self {
            msg: msg.into(),
            source: None,
        }
    }

    /// # Arguments
    /// * `msg` - The error message.
    /// * `source` - The error that caused the extraction to fail. \
    ///   It can be got back with [`Error::downcast_ref`] or [`EventErrorKind::downcast_ref`](crate::errors::EventErrorKind::downcast_ref), \
    ///   for example to handle it in the `error` observer.
    pub fn new_with_source(
        msg: impl Into<Cow<'static, str>>,
        source: impl Into<anyhow::Error>,
    ) -> Self {
        Self {
            msg: msg.into(),
            source: Some(Arc::new(source.into())),
        }
    }

    /// Returns a reference to the source of the error if it is of type `E`
    #[must_use]
    pub fn downcast_ref<E>(&self) -> Option<&E>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        self.source
            .as_ref()
            .and_then(|source| source.downcast_ref())
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!("Infallible error type should never be constructed")
    }
}
