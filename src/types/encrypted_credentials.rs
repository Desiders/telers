use serde::Deserialize;

/// Describes data required for decrypting and authenticating [`EncryptedPassportElement`](crate::types::EncryptedPassportElement). See the [`Telegram Passport Documentation`](https://core.telegram.org/passport#receiving-information) for a complete description of the data decryption and authentication processes.
/// # Documentation
/// <https://core.telegram.org/bots/api#encryptedcredentials>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct EncryptedCredentials {
    /// Base64-encoded encrypted JSON-serialized data with unique user's payload, data hashes and secrets required for [`EncryptedPassportElement`](crate::types::EncryptedPassportElement) decryption and authentication
    pub data: String,
    /// Base64-encoded data hash for data authentication
    pub hash: String,
    /// Base64-encoded secret, encrypted with the bot's public RSA key, required for data decryption
    pub secret: String,
}
