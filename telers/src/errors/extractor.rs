use std::{borrow::Cow, convert::Infallible};
use thiserror;

/// Error that can occur when extracting arguments from an event and a context to pass to a handler
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
        unreachable!()
    }
}
