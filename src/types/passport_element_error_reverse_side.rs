use serde::{Deserialize, Serialize};

/// Represents an issue with the reverse side of a document. The error is considered resolved when the file with reverse side of the document changes.
/// <https://core.telegram.org/bots/api#passportelementerrorreverseside>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorReverseSide {
    /// Error source, must be *reverse_side*
    #[serde(default = "reverse_side")]
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of 'driver_license', 'identity_card'
    #[serde(rename = "type")]
    pub element_type: String,
    /// Base64-encoded hash of the file with the reverse side of the document
    pub file_hash: String,
    /// Error message
    pub message: String,
}

impl Default for PassportElementErrorReverseSide {
    fn default() -> Self {
        Self {
            source: reverse_side(),
            element_type: String::default(),
            file_hash: String::default(),
            message: String::default(),
        }
    }
}

fn reverse_side() -> String {
    "reverse_side".to_string()
}
