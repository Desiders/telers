use serde::{Deserialize, Serialize};
/// This object represents a/an email encrypted passport element.
/// # Notes
/// This object represents an encrypted passport element from original field `email`.
/// # Documentation
/// <https://core.telegram.org/bots/api#encryptedpassportelement>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptedPassportElementEmail {
    /// User's verified email address; available only for `email` type
    pub email: Box<str>,
    /// Base64-encoded element hash for using in [`PassportElementErrorUnspecified`]
    pub hash: Box<str>,
}
impl EncryptedPassportElementEmail {
    /// Creates a new `EncryptedPassportElementEmail`.
    ///
    /// # Arguments
    /// * `email` - User's verified email address; available only for `email` type
    /// * `hash` - Base64-encoded element hash for using in [`PassportElementErrorUnspecified`]
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(email: T0, hash: T1) -> Self {
        Self {
            email: email.into(),
            hash: hash.into(),
        }
    }

    /// User's verified email address; available only for `email` type
    #[must_use]
    pub fn email<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.email = val.into();
        this
    }

    /// Base64-encoded element hash for using in [`PassportElementErrorUnspecified`]
    #[must_use]
    pub fn hash<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.hash = val.into();
        this
    }
}
