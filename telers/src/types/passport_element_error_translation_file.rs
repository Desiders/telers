use serde::{Deserialize, Serialize};
/// Represents an issue with one of the files that constitute the translation of a document. The error is considered resolved when the file changes.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrortranslationfile>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PassportElementErrorTranslationFile {
    /// Type of element of the user's Telegram Passport which has the issue, one of `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration`, `temporary_registration`
    pub r#type: Box<str>,
    /// Base64-encoded file hash
    pub file_hash: Box<str>,
    /// Error message
    pub message: Box<str>,
}
impl PassportElementErrorTranslationFile {
    /// Creates a new `PassportElementErrorTranslationFile`.
    ///
    /// # Arguments
    /// * `type` - Type of element of the user's Telegram Passport which has the issue, one of `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration`, `temporary_registration`
    /// * `file_hash` - Base64-encoded file hash
    /// * `message` - Error message
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>, T2: Into<Box<str>>>(
        r#type: T0,
        file_hash: T1,
        message: T2,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            file_hash: file_hash.into(),
            message: message.into(),
        }
    }

    /// Type of element of the user's Telegram Passport which has the issue, one of `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration`, `temporary_registration`
    #[must_use]
    pub fn r#type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.r#type = val.into();
        self
    }

    /// Base64-encoded file hash
    #[must_use]
    pub fn file_hash<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.file_hash = val.into();
        self
    }

    /// Error message
    #[must_use]
    pub fn message<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.message = val.into();
        self
    }
}
