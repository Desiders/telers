use serde::{Deserialize, Serialize};

/// Represents an issue with the reverse side of a document. The error is considered resolved when the file with reverse side of the document changes.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrorreverseside>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct PassportElementErrorReverseSide {
    /// Error source, must be *reverse_side*
    pub source: Box<str>,
    /// The section of the user's Telegram Passport which has the issue, one of 'driver_license', 'identity_card'
    #[serde(rename = "type")]
    pub element_type: Box<str>,
    /// Base64-encoded hash of the file with the reverse side of the document
    pub file_hash: Box<str>,
    /// Error message
    pub message: Box<str>,
}
