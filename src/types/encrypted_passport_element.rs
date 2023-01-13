use super::PassportFile;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes documents or other Telegram Passport elements shared with the bot by the user.
/// <https://core.telegram.org/bots/api#encryptedpassportelement>
#[skip_serializing_none]
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct EncryptedPassportElement {
    /// Element type. One of “personal_details”, “passport”, “driver_license”, “identity_card”, “internal_passport”, “address”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”, “phone_number”, “email”.
    #[serde(rename = "type")]
    pub element_type: String,
    /// Base64-encoded element hash for using in `aiogram_rs.types.passport_element_error_unspecified.PassportElementErrorUnspecified`
    pub hash: String,
    /// *Optional*. Base64-encoded encrypted Telegram Passport element data provided by the user, available for 'personal_details', 'passport', 'driver_license', 'identity_card', 'internal_passport' and 'address' types. Can be decrypted and verified using the accompanying `aiogram_rs.types.encrypted_credentials.EncryptedCredentials`.
    pub data: Option<String>,
    /// *Optional*. User's verified phone number, available only for 'phone_number' type
    pub phone_number: Option<String>,
    /// *Optional*. User's verified email address, available only for 'email' type
    pub email: Option<String>,
    /// *Optional*. Array of encrypted files with documents provided by the user, available for 'utility_bill', 'bank_statement', 'rental_agreement', 'passport_registration' and 'temporary_registration' types. Files can be decrypted and verified using the accompanying `aiogram_rs.types.encrypted_credentials.EncryptedCredentials`.
    pub files: Option<Vec<PassportFile>>,
    /// *Optional*. Encrypted file with the front side of the document, provided by the user. Available for 'passport', 'driver_license', 'identity_card' and 'internal_passport'. The file can be decrypted and verified using the accompanying `aiogram_rs.types.encrypted_credentials.EncryptedCredentials`.
    pub front_side: Option<PassportFile>,
    /// *Optional*. Encrypted file with the reverse side of the document, provided by the user. Available for 'driver_license' and 'identity_card'. The file can be decrypted and verified using the accompanying `aiogram_rs.types.encrypted_credentials.EncryptedCredentials`.
    pub reverse_side: Option<PassportFile>,
    /// *Optional*. Encrypted file with the selfie of the user holding a document, provided by the user; available for 'passport', 'driver_license', 'identity_card' and 'internal_passport'. The file can be decrypted and verified using the accompanying `aiogram_rs.types.encrypted_credentials.EncryptedCredentials`.
    pub selfie: Option<PassportFile>,
    /// *Optional*. Array of encrypted files with translated versions of documents provided by the user. Available if requested for 'passport', 'driver_license', 'identity_card', 'internal_passport', 'utility_bill', 'bank_statement', 'rental_agreement', 'passport_registration' and 'temporary_registration' types. Files can be decrypted and verified using the accompanying `aiogram_rs.types.encrypted_credentials.EncryptedCredentials`.
    pub translation: Option<Vec<PassportFile>>,
}
