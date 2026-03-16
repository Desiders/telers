use serde::{Deserialize, Serialize};
/// Describes data required for decrypting and authenticating [`crate::types::EncryptedPassportElement`]. See the Telegram Passport Documentation for a complete description of the data decryption and authentication processes.
/// # Documentation
/// <https://core.telegram.org/bots/api#encryptedcredentials>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptedCredentials {
    /// Base64-encoded encrypted JSON-serialized data with unique user's payload, data hashes and secrets required for [`crate::types::EncryptedPassportElement`] decryption and authentication
    pub data: Box<str>,
    /// Base64-encoded data hash for data authentication
    pub hash: Box<str>,
    /// Base64-encoded secret, encrypted with the bot's public RSA key, required for data decryption
    pub secret: Box<str>,
}
impl EncryptedCredentials {
    /// Creates a new `EncryptedCredentials`.
    ///
    /// # Arguments
    /// * `data` - Base64-encoded encrypted JSON-serialized data with unique user's payload, data hashes and secrets required for [`crate::types::EncryptedPassportElement`] decryption and authentication
    /// * `hash` - Base64-encoded data hash for data authentication
    /// * `secret` - Base64-encoded secret, encrypted with the bot's public RSA key, required for data decryption
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>, T2: Into<Box<str>>>(
        data: T0,
        hash: T1,
        secret: T2,
    ) -> Self {
        Self {
            data: data.into(),
            hash: hash.into(),
            secret: secret.into(),
        }
    }

    /// Base64-encoded encrypted JSON-serialized data with unique user's payload, data hashes and secrets required for [`crate::types::EncryptedPassportElement`] decryption and authentication
    #[must_use]
    pub fn data<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.data = val.into();
        this
    }

    /// Base64-encoded data hash for data authentication
    #[must_use]
    pub fn hash<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.hash = val.into();
        this
    }

    /// Base64-encoded secret, encrypted with the bot's public RSA key, required for data decryption
    #[must_use]
    pub fn secret<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.secret = val.into();
        this
    }
}
