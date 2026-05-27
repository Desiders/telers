use serde::{Deserialize, Serialize};
/// Represents an issue in an unspecified place. The error is considered resolved when new data is added.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrorunspecified>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PassportElementErrorUnspecified {
    /// Type of element of the user's Telegram Passport which has the issue
    pub r#type: Box<str>,
    /// Base64-encoded element hash
    pub element_hash: Box<str>,
    /// Error message
    pub message: Box<str>,
}
impl PassportElementErrorUnspecified {
    /// Creates a new `PassportElementErrorUnspecified`.
    ///
    /// # Arguments
    /// * `type` - Type of element of the user's Telegram Passport which has the issue
    /// * `element_hash` - Base64-encoded element hash
    /// * `message` - Error message
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>, T2: Into<Box<str>>>(
        r#type: T0,
        element_hash: T1,
        message: T2,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            element_hash: element_hash.into(),
            message: message.into(),
        }
    }

    /// Type of element of the user's Telegram Passport which has the issue
    #[must_use]
    pub fn r#type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.r#type = val.into();
        self
    }

    /// Base64-encoded element hash
    #[must_use]
    pub fn element_hash<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.element_hash = val.into();
        self
    }

    /// Error message
    #[must_use]
    pub fn message<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.message = val.into();
        self
    }
}
