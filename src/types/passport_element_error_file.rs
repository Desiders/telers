use serde::Serialize;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrorfile>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct PassportElementErrorFile {
    /// The section of the user's Telegram Passport which has the issue
    #[serde(rename = "type")]
    pub element_type: ElementType,
    /// Base64-encoded file hash
    pub file_hash: String,
    /// Error message
    pub message: String,
}

impl PassportElementErrorFile {
    #[must_use]
    pub fn new(
        element_type: ElementType,
        file_hash: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            element_type,
            file_hash: file_hash.into(),
            message: message.into(),
        }
    }

    #[must_use]
    pub fn file_hash(self, val: impl Into<String>) -> Self {
        Self {
            file_hash: val.into(),
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
    #[strum(serialize = "utility_bill")]
    UtilityBill,
    #[strum(serialize = "bank_statement")]
    BankStatement,
    #[strum(serialize = "rental_agreement")]
    RentalAgreement,
    #[strum(serialize = "passport_registration")]
    PassportRegistration,
    #[strum(serialize = "temporary_registration")]
    TemporaryRegistration,
}
