use serde::{Deserialize, Serialize};

/// Represents an issue with the front side of a document. The error is considered resolved when the file with the front side of the document changes.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrorfrontside>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct PassportElementErrorFrontSide {
    /// Error source, must be *front_side*
    pub source: Box<str>,
    /// The section of the user's Telegram Passport which has the issue, one of 'passport', 'driver_license', 'identity_card', 'internal_passport'
    #[serde(rename = "type")]
    pub element_type: Box<str>,
    /// Base64-encoded hash of the file with the front side of the document
    pub file_hash: Box<str>,
    /// Error message
    pub message: Box<str>,
}
