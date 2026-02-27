use serde::{Deserialize, Serialize};
/// This object represents a/an address encrypted passport element.
/// # Notes
/// This object represents an encrypted passport element from original field `address`.
/// # Documentation
/// <https://core.telegram.org/bots/api#encryptedpassportelement>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptedPassportElementAddress {
    /// Base64-encoded encrypted Telegram Passport element data provided by the user; available only for `personal_details`, `passport`, `driver_license`, `identity_card`, `internal_passport` and `address` types. Can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    pub data: Box<str>,
    /// Base64-encoded element hash for using in [`PassportElementErrorUnspecified`]
    pub hash: Box<str>,
}
impl EncryptedPassportElementAddress {
    /// Creates a new `EncryptedPassportElementAddress`.
    ///
    /// # Arguments
    /// * `data` - Base64-encoded encrypted Telegram Passport element data provided by the user; available only for `personal_details`, `passport`, `driver_license`, `identity_card`, `internal_passport` and `address` types. Can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    /// * `hash` - Base64-encoded element hash for using in [`PassportElementErrorUnspecified`]
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(data: T0, hash: T1) -> Self {
        Self {
            data: data.into(),
            hash: hash.into(),
        }
    }

    /// Base64-encoded encrypted Telegram Passport element data provided by the user; available only for `personal_details`, `passport`, `driver_license`, `identity_card`, `internal_passport` and `address` types. Can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    #[must_use]
    pub fn data<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.data = val.into();
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
