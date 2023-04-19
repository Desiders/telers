use super::ExtractionError;

use std::borrow::Cow;
use thiserror;

/// This error occurs when the update type cannot be converted to the desired type.
///
/// For example, if you try to convert an update to a [`crate::types::Message`] type,
/// but the update represents a [`crate::types::CallbackQuery`], you will get this error.
#[derive(thiserror::Error, Debug)]
#[error("Convert update to type error: {msg}")]
pub struct ConvertUpdateToType {
    msg: Cow<'static, str>,
}

impl ConvertUpdateToType {
    pub fn new<T>(msg: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self { msg: msg.into() }
    }
}

impl From<ConvertUpdateToType> for ExtractionError {
    fn from(err: ConvertUpdateToType) -> Self {
        Self::new(err.msg)
    }
}
