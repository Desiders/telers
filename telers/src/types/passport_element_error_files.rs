use serde::Serialize;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// Represents an issue with a list of scans. The error is considered resolved when the list of files containing the scans changes.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrorfiles>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct PassportElementErrorFiles {
    /// The section of the user's Telegram Passport which has the issue
    #[serde(rename = "type")]
    pub element_type: ElementType,
    /// List of base64-encoded file hashes
    pub file_hashes: Vec<String>,
    /// Error message
    pub message: String,
}

impl PassportElementErrorFiles {
    #[must_use]
    pub fn new<T, I>(element_type: ElementType, file_hashes: I, message: impl Into<String>) -> Self
    where
        T: Into<String>,
        I: IntoIterator<Item = T>,
    {
        Self {
            element_type,
            file_hashes: file_hashes.into_iter().map(Into::into).collect(),
            message: message.into(),
        }
    }

    #[must_use]
    pub fn file_hash(self, val: impl Into<String>) -> Self {
        Self {
            file_hashes: self
                .file_hashes
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn file_hashes<T, I>(self, val: I) -> Self
    where
        T: Into<String>,
        I: IntoIterator<Item = T>,
    {
        Self {
            file_hashes: self
                .file_hashes
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
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
