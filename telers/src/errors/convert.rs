//! Conversion errors for Telegram objects.
pub use telers_types::errors::convert::ConvertToType;

impl From<ConvertToType> for super::ExtractionError {
    fn from(err: ConvertToType) -> Self {
        Self::new(err.to_string())
    }
}
