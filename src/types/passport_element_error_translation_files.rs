use serde::Deserialize;

/// Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrortranslationfiles>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct PassportElementErrorTranslationFiles {
    /// Error source, must be *translation_files*
    pub source: String,
    /// Type of element of the user's Telegram Passport which has the issue, one of 'passport', 'driver_license', 'identity_card', 'internal_passport', 'utility_bill', 'bank_statement', 'rental_agreement', 'passport_registration', 'temporary_registration'
    #[serde(rename = "type")]
    pub element_type: String,
    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
    /// Error message
    pub message: String,
}
