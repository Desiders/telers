//! This module contains the error [`ConvertToType`] that can occur when type be converted to the desired type.
//! For example, if you try to convert an [`Update`] to a [`Message`] type, but the update represents a [`CallbackQuery`], you got this error.
//!
//! This error type used usually in [`FromEventAndContext`] trait implementations when the type conversion fails.
//!
//! [`Update`]: crate::types::Update
//! [`Message`]: crate::types::Message
//! [`CallbackQuery`]: crate::types::CallbackQuery
//! [`FromEventAndContext`]: crate::extractors::FromEventAndContext

use super::ExtractionError;

/// This error can occur when type be converted to the desired type.
/// For example, if you try to convert an [`crate::types::Update`] to a [`crate::types::Message`] type,
/// but the update represents a [`crate::types::CallbackQuery`], you got this error.
#[derive(Debug, thiserror::Error)]
#[error("Can't convert from `{from_raw_type}` to `{to_raw_type}`")]
pub struct ConvertToType {
    from_raw_type: &'static str,
    to_raw_type: &'static str,
}

impl ConvertToType {
    /// # Arguments
    /// * `from_raw_type` - The type from which the conversion is performed.
    /// * `to_raw_type` - The type to which the conversion is performed.
    #[must_use]
    pub const fn new(from_raw_type: &'static str, to_raw_type: &'static str) -> Self {
        Self {
            from_raw_type,
            to_raw_type,
        }
    }
}

impl From<ConvertToType> for ExtractionError {
    fn from(err: ConvertToType) -> Self {
        Self::new(err.to_string())
    }
}
