use serde::{Deserialize, Serialize};

/// Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrorselfie>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct PassportElementErrorSelfie {
    /// Error source, must be *selfie*
    pub source: Box<str>,
    /// The section of the user's Telegram Passport which has the issue, one of 'passport', 'driver_license', 'identity_card', 'internal_passport'
    #[serde(rename = "type")]
    pub element_type: Box<str>,
    /// Base64-encoded hash of the file with the selfie
    pub file_hash: Box<str>,
    /// Error message
    pub message: Box<str>,
}
