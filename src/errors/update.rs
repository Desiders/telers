use super::ExtractionError;

use std::borrow::Cow;

/// Error that can occur when update can't be converted to the desired type.
///
/// For example, if you try to convert an [`crate::types::Update`] to a [`crate::types::Message`] type,
/// but the update represents a [`crate::types::CallbackQuery`], you got this error.
#[derive(Debug, thiserror::Error)]
#[error("Convert update to type `{raw_type}` error")]
pub struct ConvertUpdateToType {
    raw_type: Cow<'static, str>,
}

impl ConvertUpdateToType {
    pub fn new<T>(raw_type: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self {
            raw_type: raw_type.into(),
        }
    }
}

impl From<ConvertUpdateToType> for ExtractionError {
    fn from(err: ConvertUpdateToType) -> Self {
        Self::new(err.to_string())
    }
}
