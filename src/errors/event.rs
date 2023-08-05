use super::{ExtractionError, HandlerError, MiddlewareError};

use std::fmt::Debug;
use thiserror;

/// Possible errors that can occur when processing an event
#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error(transparent)]
    Extraction(#[from] ExtractionError),
    #[error(transparent)]
    Handler(#[from] HandlerError),
    #[error(transparent)]
    Middleware(#[from] MiddlewareError),
}