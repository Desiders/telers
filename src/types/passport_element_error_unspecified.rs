use serde::{Deserialize, Serialize};

/// Represents an issue in an unspecified place. The error is considered resolved when new data is added.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrorunspecified>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct PassportElementErrorUnspecified {
    /// Error source, must be *unspecified*
    pub source: Box<str>,
    /// Type of element of the user's Telegram Passport which has the issue
    #[serde(rename = "type")]
    pub element_type: Box<str>,
    /// Base64-encoded element hash
    pub element_hash: Box<str>,
    /// Error message
    pub message: Box<str>,
}
