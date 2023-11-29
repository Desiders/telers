use serde::Serialize;

/// Represents an issue in an unspecified place. The error is considered resolved when new data is added.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrorunspecified>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct PassportElementErrorUnspecified {
    /// Type of element of the user's Telegram Passport which has the issue
    #[serde(rename = "type")]
    pub element_type: String,
    /// Base64-encoded element hash
    pub element_hash: String,
    /// Error message
    pub message: String,
}

impl PassportElementErrorUnspecified {
    #[must_use]
    pub fn new(
        element_type: impl Into<String>,
        element_hash: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            element_type: element_type.into(),
            element_hash: element_hash.into(),
            message: message.into(),
        }
    }

    #[must_use]
    pub fn element_type(self, val: impl Into<String>) -> Self {
        Self {
            element_type: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn element_hash(self, val: impl Into<String>) -> Self {
        Self {
            element_hash: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn message(self, val: impl Into<String>) -> Self {
        Self {
            message: val.into(),
            ..self
        }
    }
}
