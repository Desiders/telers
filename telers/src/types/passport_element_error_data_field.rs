use serde::{Deserialize, Serialize};
/// Represents an issue in one of the data fields that was provided by the user. The error is considered resolved when the field's value changes.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrordatafield>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PassportElementErrorDataField {
    /// The section of the user's Telegram Passport which has the error, one of `personal_details`, `passport`, `driver_license`, `identity_card`, `internal_passport`, `address`
    pub r#type: Box<str>,
    /// Name of the data field which has the error
    pub field_name: Box<str>,
    /// Base64-encoded data hash
    pub data_hash: Box<str>,
    /// Error message
    pub message: Box<str>,
}
impl PassportElementErrorDataField {
    /// Creates a new `PassportElementErrorDataField`.
    ///
    /// # Arguments
    /// * `type` - The section of the user's Telegram Passport which has the error, one of `personal_details`, `passport`, `driver_license`, `identity_card`, `internal_passport`, `address`
    /// * `field_name` - Name of the data field which has the error
    /// * `data_hash` - Base64-encoded data hash
    /// * `message` - Error message
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>, T2: Into<Box<str>>, T3: Into<Box<str>>>(
        r#type: T0,
        field_name: T1,
        data_hash: T2,
        message: T3,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            field_name: field_name.into(),
            data_hash: data_hash.into(),
            message: message.into(),
        }
    }

    /// The section of the user's Telegram Passport which has the error, one of `personal_details`, `passport`, `driver_license`, `identity_card`, `internal_passport`, `address`
    #[must_use]
    pub fn r#type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.r#type = val.into();
        self
    }

    /// Name of the data field which has the error
    #[must_use]
    pub fn field_name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.field_name = val.into();
        self
    }

    /// Base64-encoded data hash
    #[must_use]
    pub fn data_hash<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.data_hash = val.into();
        self
    }

    /// Error message
    #[must_use]
    pub fn message<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.message = val.into();
        self
    }
}
