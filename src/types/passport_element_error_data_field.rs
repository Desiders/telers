use serde::Deserialize;

/// Represents an issue in one of the data fields that was provided by the user. The error is considered resolved when the field's value changes.
/// <https://core.telegram.org/bots/api#passportelementerrordatafield>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct PassportElementErrorDataField {
    /// Error source, must be *data*
    pub source: String,
    /// The section of the user's Telegram Passport which has the error, one of 'personal_details', 'passport', 'driver_license', 'identity_card', 'internal_passport', 'address'
    #[serde(rename = "type")]
    pub element_type: String,
    /// Base64-encoded data hash
    pub data_hash: String,
    /// Error message
    pub message: String,
}
