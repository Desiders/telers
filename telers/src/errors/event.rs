//! This module contains the error types that can occur when processing an event
//!
//! Possible errors that can occur when processing an event:
//! - [`ExtractionError`] - An error that can occur when extracting arguments from an event and a context to pass to a handler
//! - [`HandlerError`] - An error that can occur when processing a handler
//! - [`MiddlewareError`] - An error that can occur when processing a middleware (may wrap [`HandlerError`])
//! - [`FilterError`] - An error that can occur when processing a filter

use super::{ExtractionError, FilterError, HandlerError, MiddlewareError};

use std::{
    any::Any,
    fmt::{Debug, Display},
};
use thiserror;

/// Possible errors that can occur when processing an event:
/// - [`ExtractionError`] - An error that can occur when extracting arguments from an event and a context to pass to a handler
/// - [`HandlerError`] - An error that can occur when processing a handler
/// - [`MiddlewareError`] - An error that can occur when processing a middleware (may wrap [`HandlerError`])
/// - [`FilterError`] - An error that can occur when processing a filter
#[derive(Debug, Clone, thiserror::Error)]
pub enum ErrorKind {
    #[error(transparent)]
    Extraction(#[from] ExtractionError),
    #[error(transparent)]
    Handler(#[from] HandlerError),
    #[error(transparent)]
    Middleware(#[from] MiddlewareError),
    #[error(transparent)]
    Filter(#[from] FilterError),
}

impl ErrorKind {
    #[must_use]
    pub fn downcast_ref<E>(&self) -> Option<&E>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        match self {
            Self::Extraction(err) => (err as &dyn Any).downcast_ref(),
            Self::Handler(err) => err.downcast_ref(),
            Self::Middleware(err) => err.downcast_ref(),
            Self::Filter(err) => err.downcast_ref(),
        }
    }
}
