use super::ExtractionError;

use std::borrow::Cow;
use thiserror;

#[derive(thiserror::Error, Debug)]
#[error("Convert update to type error: {msg}")]
pub struct ConvertUpdateToType {
    msg: Cow<'static, str>,
}

impl ConvertUpdateToType {
    pub fn new<T: Into<Cow<'static, str>>>(msg: T) -> Self {
        Self { msg: msg.into() }
    }
}

impl From<ConvertUpdateToType> for ExtractionError {
    fn from(err: ConvertUpdateToType) -> Self {
        Self::new(err.msg)
    }
}
