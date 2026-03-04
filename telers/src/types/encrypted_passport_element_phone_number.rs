use serde::{Deserialize, Serialize};
/// This object represents a/an phone number encrypted passport element.
/// # Notes
/// This object represents an encrypted passport element from original field `phone_number`.
/// # Documentation
/// <https://core.telegram.org/bots/api#encryptedpassportelement>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptedPassportElementPhoneNumber {
    /// User's verified phone number; available only for `phone_number` type
    pub phone_number: Box<str>,
    /// Base64-encoded element hash for using in [`crate::types::PassportElementErrorUnspecified`]
    pub hash: Box<str>,
}
impl EncryptedPassportElementPhoneNumber {
    /// Creates a new `EncryptedPassportElementPhoneNumber`.
    ///
    /// # Arguments
    /// * `phone_number` - User's verified phone number; available only for `phone_number` type
    /// * `hash` - Base64-encoded element hash for using in [`crate::types::PassportElementErrorUnspecified`]
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(phone_number: T0, hash: T1) -> Self {
        Self {
            phone_number: phone_number.into(),
            hash: hash.into(),
        }
    }

    /// User's verified phone number; available only for `phone_number` type
    #[must_use]
    pub fn phone_number<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.phone_number = val.into();
        this
    }

    /// Base64-encoded element hash for using in [`crate::types::PassportElementErrorUnspecified`]
    #[must_use]
    pub fn hash<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.hash = val.into();
        this
    }
}
