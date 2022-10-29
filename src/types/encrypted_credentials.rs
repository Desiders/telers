use serde::{Deserialize, Serialize};

/// Describes data required for decrypting and authenticating `aiogram_rs.types.encrypted_passport_element.EncryptedPassportElement`. See the `Telegram Passport Documentation <https://core.telegram.org/passport#receiving-information>`_ for a complete description of the data decryption and authentication processes.
/// <https://core.telegram.org/bots/api#encryptedcredentials>_
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct EncryptedCredentials {
    /// Base64-encoded encrypted JSON-serialized data with unique user's payload, data hashes and secrets required for `aiogram_rs.types.encrypted_passport_element.EncryptedPassportElement` decryption and authentication
    pub data: String,
    /// Base64-encoded data hash for data authentication
    pub hash: String,
    /// Base64-encoded secret, encrypted with the bot's public RSA key, required for data decryption
    pub secret: String,
}
