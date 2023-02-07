use std::{borrow::Cow, convert::Infallible};
use thiserror;

#[derive(thiserror::Error, Debug)]
#[error("Extraction error: {msg}")]
pub struct Error {
    msg: Cow<'static, str>,
}

impl Error {
    pub fn new<T: Into<Cow<'static, str>>>(msg: T) -> Self {
        Self { msg: msg.into() }
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
