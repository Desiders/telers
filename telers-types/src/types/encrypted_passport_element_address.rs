#[cfg(not(feature = "deser"))]
use crate::deserialization::encrypted_passport_element as decoder;

/// This object represents a/an address encrypted passport element.
/// # Notes
/// This object represents an encrypted passport element from original field `address`.
/// # Documentation
/// <https://core.telegram.org/bots/api#encryptedpassportelement>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct EncryptedPassportElementAddress {
    /// Base64-encoded encrypted Telegram Passport element data provided by the user; available only for `personal_details`, `passport`, `driver_license`, `identity_card`, `internal_passport` and `address` types. Can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    pub data: Box<str>,
    /// Base64-encoded element hash for using in [`crate::types::PassportElementErrorUnspecified`]
    pub hash: Box<str>,
}

impl EncryptedPassportElementAddress {
    /// Creates a new `EncryptedPassportElementAddress`.
    ///
    /// # Arguments
    /// * `data` - Base64-encoded encrypted Telegram Passport element data provided by the user; available only for `personal_details`, `passport`, `driver_license`, `identity_card`, `internal_passport` and `address` types. Can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    /// * `hash` - Base64-encoded element hash for using in [`crate::types::PassportElementErrorUnspecified`]
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(data: T0, hash: T1) -> Self {
        Self {
            data: data.into(),
            hash: hash.into(),
        }
    }

    /// Base64-encoded encrypted Telegram Passport element data provided by the user; available only for `personal_details`, `passport`, `driver_license`, `identity_card`, `internal_passport` and `address` types. Can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    #[must_use]
    pub fn data<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.data = val.into();
        self
    }

    /// Base64-encoded element hash for using in [`crate::types::PassportElementErrorUnspecified`]
    #[must_use]
    pub fn hash<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.hash = val.into();
        self
    }
}

#[cfg(not(feature = "deser"))]
impl<'de> serde::Deserialize<'de> for EncryptedPassportElementAddress {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let decoded = decoder::deserialize_content(deserializer, &decoder::ADDRESS)?;
        Ok(Self {
            data: decoded.content,
            hash: decoded.common.hash,
        })
    }
}
