use serde::Serialize;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// Represents an issue in one of the data fields that was provided by the user. The error is considered resolved when the field's value changes.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrordatafield>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct PassportElementErrorDataField {
    /// The section of the user's Telegram Passport which has the error
    #[serde(rename = "type")]
    pub element_type: ElementType,
    /// Name of the data field which has the error
    pub field_name: String,
    /// Base64-encoded data hash
    pub data_hash: String,
    /// Error message
    pub message: String,
}

impl PassportElementErrorDataField {
    #[must_use]
    pub fn new(
        element_type: ElementType,
        field_name: impl Into<String>,
        data_hash: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            element_type,
            field_name: field_name.into(),
            data_hash: data_hash.into(),
            message: message.into(),
        }
    }

    #[must_use]
    pub fn field_name(self, val: impl Into<String>) -> Self {
        Self {
            field_name: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn data_hash(self, val: impl Into<String>) -> Self {
        Self {
            data_hash: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn message(self, val: impl Into<String>) -> Self {
        Self {
            message: val.into(),
            ..self
        }
    }
}

#[derive(
    Debug, Display, Clone, Copy, PartialEq, Eq, Hash, Serialize, EnumString, AsRefStr, IntoStaticStr,
)]
#[serde(rename_all = "snake_case")]
pub enum ElementType {
    #[strum(serialize = "personal_details")]
    PersonalDetails,
    #[strum(serialize = "passport")]
    Passport,
    #[strum(serialize = "driver_license")]
    DriverLicense,
    #[strum(serialize = "identity_card")]
    IdentityCard,
    #[strum(serialize = "internal_passport")]
    InternalPassport,
    #[strum(serialize = "address")]
    Address,
}
