use serde::{Deserialize, Serialize};

/// Represents an issue with the front side of a document. The error is considered resolved when the file with the front side of the document changes.
/// <https://core.telegram.org/bots/api#passportelementerrorfrontside>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorFrontSide {
    /// Error source, must be *front_side*
    #[serde(default = "front_side")]
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of 'passport', 'driver_license', 'identity_card', 'internal_passport'
    #[serde(rename = "type")]
    pub element_type: String,
    /// Base64-encoded hash of the file with the front side of the document
    pub file_hash: String,
    /// Error message
    pub message: String,
}

fn front_side() -> String {
    "front_side".to_string()
}
