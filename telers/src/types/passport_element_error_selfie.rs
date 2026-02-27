use serde::{Deserialize, Serialize};
/// Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrorselfie>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PassportElementErrorSelfie {
    /// The section of the user's Telegram Passport which has the issue, one of `passport`, `driver_license`, `identity_card`, `internal_passport`
    pub r#type: Box<str>,
    /// Base64-encoded hash of the file with the selfie
    pub file_hash: Box<str>,
    /// Error message
    pub message: Box<str>,
}
impl PassportElementErrorSelfie {
    /// Creates a new `PassportElementErrorSelfie`.
    ///
    /// # Arguments
    /// * `type` - The section of the user's Telegram Passport which has the issue, one of `passport`, `driver_license`, `identity_card`, `internal_passport`
    /// * `file_hash` - Base64-encoded hash of the file with the selfie
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
    pub fn r#type<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.r#type = val.into();
        this
    }

    /// Base64-encoded hash of the file with the selfie
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
