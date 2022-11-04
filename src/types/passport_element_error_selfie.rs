use serde::{Deserialize, Serialize};

/// Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
/// <https://core.telegram.org/bots/api#passportelementerrorselfie>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorSelfie {
    /// Error source, must be *selfie*
    #[serde(default = "selfie")]
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of 'passport', 'driver_license', 'identity_card', 'internal_passport'
    #[serde(rename = "type")]
    pub element_type: String,
    /// Base64-encoded hash of the file with the selfie
    pub file_hash: String,
    /// Error message
    pub message: String,
}

impl Default for PassportElementErrorSelfie {
    fn default() -> Self {
        Self {
            source: selfie(),
            element_type: String::default(),
            file_hash: String::default(),
            message: String::default(),
        }
    }
}

fn selfie() -> String {
    "selfie".to_string()
}
