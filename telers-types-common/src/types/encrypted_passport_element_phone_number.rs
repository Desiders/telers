/// This object represents a/an phone number encrypted passport element.
/// # Notes
/// This object represents an encrypted passport element from original field `phone_number`.
/// # Documentation
/// <https://core.telegram.org/bots/api#encryptedpassportelement>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct EncryptedPassportElementPhoneNumber {
    /// User's verified phone number; available only for `phone_number` type
    pub phone_number: Box<str>,
    /// Base64-encoded element hash for using in [`telers_types_resources::types::PassportElementErrorUnspecified`](https://docs.rs/telers-types-resources/latest/telers_types_resources/types/passport_element_error_unspecified/struct.PassportElementErrorUnspecified.html)
    pub hash: Box<str>,
}
impl EncryptedPassportElementPhoneNumber {
    /// Creates a new `EncryptedPassportElementPhoneNumber`.
    ///
    /// # Arguments
    /// * `phone_number` - User's verified phone number; available only for `phone_number` type
    /// * `hash` - Base64-encoded element hash for using in [`telers_types_resources::types::PassportElementErrorUnspecified`](https://docs.rs/telers-types-resources/latest/telers_types_resources/types/passport_element_error_unspecified/struct.PassportElementErrorUnspecified.html)
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

    /// Base64-encoded element hash for using in [`telers_types_resources::types::PassportElementErrorUnspecified`](https://docs.rs/telers-types-resources/latest/telers_types_resources/types/passport_element_error_unspecified/struct.PassportElementErrorUnspecified.html)
    #[must_use]
    pub fn hash<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.hash = val.into();
        self
    }
}
