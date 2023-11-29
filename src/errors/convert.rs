use super::ExtractionError;

/// Error that can occur when type be converted to the desired type.
///
/// For example, if you try to convert an [`crate::types::Update`] to a [`crate::types::Message`] type,
/// but the update represents a [`crate::types::CallbackQuery`], you got this error,
/// or if you try to convert a [`crate::types::Message`] to a [`crate::types::MessageText`] type,
/// but the message contains a [`crate::types::MessageAnimation`], you got this error.
#[derive(Debug, thiserror::Error)]
#[error("Convert type from {from_raw_type} to {to_raw_type}")]
pub struct ConvertToType {
    from_raw_type: &'static str,
    to_raw_type: &'static str,
}

impl ConvertToType {
    #[must_use] pub fn new(from_raw_type: &'static str, to_raw_type: &'static str) -> Self {
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
