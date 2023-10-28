use serde::{Deserialize, Serialize};

/// Represents an issue in one of the data fields that was provided by the user. The error is considered resolved when the field's value changes.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrordatafield>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct PassportElementErrorDataField {
    /// Error source, must be *data*
    pub source: Box<str>,
    /// The section of the user's Telegram Passport which has the error, one of 'personal_details', 'passport', 'driver_license', 'identity_card', 'internal_passport', 'address'
    #[serde(rename = "type")]
    pub element_type: Box<str>,
    /// Base64-encoded data hash
    pub data_hash: Box<str>,
    /// Error message
    pub message: Box<str>,
}
