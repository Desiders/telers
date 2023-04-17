use super::PassportFile;

use serde::Deserialize;

/// Describes documents or other Telegram Passport elements shared with the bot by the user.
/// # Documentation
/// <https://core.telegram.org/bots/api#encryptedpassportelement>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct EncryptedPassportElement {
    /// Element type. One of “personal_details”, “passport”, “driver_license”, “identity_card”, “internal_passport”, “address”, “utility_bill”, “bank_statement”, “rental_agreement”, “passport_registration”, “temporary_registration”, “phone_number”, “email”.
    #[serde(rename = "type")]
    pub element_type: String,
    /// Base64-encoded element hash for using in [`PassportElementErrorUnspecified`](crate::types::PassportElementErrorUnspecified)
    pub hash: String,
    /// Base64-encoded encrypted Telegram Passport element data provided by the user, available for 'personal_details', 'passport', 'driver_license', 'identity_card', 'internal_passport' and 'address' types. Can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub data: Option<String>,
    /// User's verified phone number, available only for 'phone_number' type
    pub phone_number: Option<String>,
    /// User's verified email address, available only for 'email' type
    pub email: Option<String>,
    /// Array of encrypted files with documents provided by the user, available for 'utility_bill', 'bank_statement', 'rental_agreement', 'passport_registration' and 'temporary_registration' types. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub files: Option<Vec<PassportFile>>,
    /// Encrypted file with the front side of the document, provided by the user. Available for 'passport', 'driver_license', 'identity_card' and 'internal_passport'. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub front_side: Option<PassportFile>,
    /// Encrypted file with the reverse side of the document, provided by the user. Available for 'driver_license' and 'identity_card'. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub reverse_side: Option<PassportFile>,
    /// Encrypted file with the selfie of the user holding a document, provided by the user; available for 'passport', 'driver_license', 'identity_card' and 'internal_passport'. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub selfie: Option<PassportFile>,
    /// Array of encrypted files with translated versions of documents provided by the user. Available if requested for 'passport', 'driver_license', 'identity_card', 'internal_passport', 'utility_bill', 'bank_statement', 'rental_agreement', 'passport_registration' and 'temporary_registration' types. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub translation: Option<Vec<PassportFile>>,
}
