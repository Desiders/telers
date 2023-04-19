use std::{borrow::Cow, convert::Infallible};
use thiserror;

/// This struct represents error that can occur in the extraction process
#[derive(thiserror::Error, Debug)]
#[error("Extraction error: {msg}")]
pub struct Error {
    msg: Cow<'static, str>,
}

impl Error {
    pub fn new<T>(msg: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self { msg: msg.into() }
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
