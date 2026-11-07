/// This object represents a/an phone number encrypted passport element.
/// # Notes
/// This object represents an encrypted passport element from original field `phone_number`.
/// # Documentation
/// <https://core.telegram.org/bots/api#encryptedpassportelement>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
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
    pub fn phone_number<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.phone_number = val.into();
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
impl<'de> serde::Deserialize<'de> for EncryptedPassportElementPhoneNumber {
    #[allow(clippy::too_many_lines)]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let meta = crate::deserialization::Meta {
            name: "EncryptedPassportElementPhoneNumber",
            fields: &["phone_number", "hash"],
            specific: &["phone_number"],
            sequence: "struct EncryptedPassportElementPhoneNumber with 2 elements",
        };
        let (common, (value_phone_number,)) =
            crate::deserialization::encrypted_passport_element::read_0::<D, Box<str>>(
                deserializer,
                meta,
            )?;
        Ok(Self {
            phone_number: value_phone_number,
            hash: common.hash,
        })
    }
}
