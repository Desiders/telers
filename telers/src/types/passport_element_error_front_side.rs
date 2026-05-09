use serde::{Deserialize, Serialize};
/// Represents an issue with the front side of a document. The error is considered resolved when the file with the front side of the document changes.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrorfrontside>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PassportElementErrorFrontSide {
    /// The section of the user's Telegram Passport which has the issue, one of `passport`, `driver_license`, `identity_card`, `internal_passport`
    pub r#type: Box<str>,
    /// Base64-encoded hash of the file with the front side of the document
    pub file_hash: Box<str>,
    /// Error message
    pub message: Box<str>,
}
impl PassportElementErrorFrontSide {
    /// Creates a new `PassportElementErrorFrontSide`.
    ///
    /// # Arguments
    /// * `type` - The section of the user's Telegram Passport which has the issue, one of `passport`, `driver_license`, `identity_card`, `internal_passport`
    /// * `file_hash` - Base64-encoded hash of the file with the front side of the document
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

    /// The section of the user's Telegram Passport which has the issue, one of `passport`, `driver_license`, `identity_card`, `internal_passport`
    #[must_use]
    pub fn r#type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.r#type = val.into();
        self
    }

    /// Base64-encoded hash of the file with the front side of the document
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
