#[cfg(not(feature = "deser"))]
use crate::deserialization::encrypted_passport_element as decoder;

/// This object represents a/an email encrypted passport element.
/// # Notes
/// This object represents an encrypted passport element from original field `email`.
/// # Documentation
/// <https://core.telegram.org/bots/api#encryptedpassportelement>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct EncryptedPassportElementEmail {
    /// User's verified email address; available only for `email` type
    pub email: Box<str>,
    /// Base64-encoded element hash for using in [`crate::types::PassportElementErrorUnspecified`]
    pub hash: Box<str>,
}

impl EncryptedPassportElementEmail {
    /// Creates a new `EncryptedPassportElementEmail`.
    ///
    /// # Arguments
    /// * `email` - User's verified email address; available only for `email` type
    /// * `hash` - Base64-encoded element hash for using in [`crate::types::PassportElementErrorUnspecified`]
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(email: T0, hash: T1) -> Self {
        Self {
            email: email.into(),
            hash: hash.into(),
        }
    }

    /// User's verified email address; available only for `email` type
    #[must_use]
    pub fn email<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.email = val.into();
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
impl<'de> serde::Deserialize<'de> for EncryptedPassportElementEmail {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let decoded = decoder::deserialize_content(deserializer, &decoder::EMAIL)?;
        Ok(Self {
            email: decoded.content,
            hash: decoded.common.hash,
        })
    }
}
