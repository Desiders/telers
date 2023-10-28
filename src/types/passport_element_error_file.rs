use serde::{Deserialize, Serialize};

/// Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrorfile>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct PassportElementErrorFile {
    /// Error source, must be *file*
    pub source: Box<str>,
    /// The section of the user's Telegram Passport which has the issue, one of 'utility_bill', 'bank_statement', 'rental_agreement', 'passport_registration', 'temporary_registration'
    #[serde(rename = "type")]
    pub element_type: Box<str>,
    /// Base64-encoded file hash
    pub file_hash: Box<str>,
    /// Error message
    pub message: Box<str>,
}
