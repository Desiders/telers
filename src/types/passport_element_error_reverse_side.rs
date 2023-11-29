use serde::Serialize;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// Represents an issue with the reverse side of a document. The error is considered resolved when the file with reverse side of the document changes.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrorreverseside>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct PassportElementErrorReverseSide {
    /// The section of the user's Telegram Passport which has the issue
    #[serde(rename = "type")]
    pub element_type: ElementType,
    /// Base64-encoded hash of the file with the reverse side of the document
    pub file_hash: String,
    /// Error message
    pub message: String,
}

impl PassportElementErrorReverseSide {
    #[must_use]
    pub fn new(
        element_type: ElementType,
        file_hash: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            element_type,
            file_hash: file_hash.into(),
            message: message.into(),
        }
    }

    #[must_use]
    pub fn file_hash(self, val: impl Into<String>) -> Self {
        Self {
            file_hash: val.into(),
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

#[derive(
    Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, EnumString, AsRefStr, IntoStaticStr,
)]
#[serde(rename_all = "snake_case")]
pub enum ElementType {
    #[strum(serialize = "driver_license")]
    DriverLicense,
    #[strum(serialize = "identity_card")]
    IdentityCard,
}
