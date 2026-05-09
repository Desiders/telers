use serde::{Deserialize, Serialize};
/// Describes why a request was unsuccessful.
/// # Documentation
/// <https://core.telegram.org/bots/api#responseparameters>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseParameters {
    /// The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,
    /// In case of exceeding flood control, the number of seconds left to wait before the request can be repeated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<i64>,
}
impl ResponseParameters {
    /// Creates a new `ResponseParameters`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            migrate_to_chat_id: None,
            retry_after: None,
        }
    }

    /// The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn migrate_to_chat_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.migrate_to_chat_id = Some(val.into());
        self
    }

    /// The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn migrate_to_chat_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.migrate_to_chat_id = val.map(Into::into);
        self
    }

    /// In case of exceeding flood control, the number of seconds left to wait before the request can be repeated
    #[must_use]
    pub fn retry_after<T: Into<i64>>(mut self, val: T) -> Self {
        self.retry_after = Some(val.into());
        self
    }

    /// In case of exceeding flood control, the number of seconds left to wait before the request can be repeated
    #[must_use]
    pub fn retry_after_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.retry_after = val.map(Into::into);
        self
    }
}
impl Default for ResponseParameters {
    fn default() -> Self {
        Self::new()
    }
}
