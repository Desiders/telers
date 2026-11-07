/// Represents an issue in one of the data fields that was provided by the user. The error is considered resolved when the field's value changes.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrordatafield>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
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
#[cfg(not(feature = "deser"))]
impl<'de> serde::Deserialize<'de> for PassportElementErrorDataField {
    #[allow(clippy::too_many_lines)]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let meta = crate::deserialization::Meta {
            name: "PassportElementErrorDataField",
            fields: &["type", "field_name", "data_hash", "message"],
            specific: &["field_name", "data_hash"],
            sequence: "struct PassportElementErrorDataField with 4 elements",
        };
        let (common, (value_field_name, value_data_hash)) =
            crate::deserialization::passport_element_error::read_0::<D, Box<str>, Box<str>>(
                deserializer,
                meta,
            )?;
        Ok(Self {
            r#type: common.r#type,
            field_name: value_field_name,
            data_hash: value_data_hash,
            message: common.message,
        })
    }
}
