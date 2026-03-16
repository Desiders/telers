use serde::{Deserialize, Serialize};
/// Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrorfile>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PassportElementErrorFile {
    /// The section of the user's Telegram Passport which has the issue, one of `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration`, `temporary_registration`
    pub r#type: Box<str>,
    /// Base64-encoded file hash
    pub file_hash: Box<str>,
    /// Error message
    pub message: Box<str>,
}
impl PassportElementErrorFile {
    /// Creates a new `PassportElementErrorFile`.
    ///
    /// # Arguments
    /// * `type` - The section of the user's Telegram Passport which has the issue, one of `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration`, `temporary_registration`
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

    /// The section of the user's Telegram Passport which has the issue, one of `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration`, `temporary_registration`
    #[must_use]
    pub fn r#type<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.r#type = val.into();
        this
    }

    /// Base64-encoded file hash
    #[must_use]
    pub fn file_hash<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.file_hash = val.into();
        this
    }

    /// Error message
    #[must_use]
    pub fn message<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.message = val.into();
        this
    }
}
