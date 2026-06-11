use serde::{Deserialize, Serialize};
/// Describes an inline message to be sent by a user of a Mini App.
/// # Documentation
/// <https://core.telegram.org/bots/api#preparedinlinemessage>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PreparedInlineMessage {
    /// Unique identifier of the prepared message
    pub id: Box<str>,
    /// Expiration date of the prepared message, in Unix time. Expired prepared messages can no longer be used.
    pub expiration_date: i64,
}
impl PreparedInlineMessage {
    /// Creates a new `PreparedInlineMessage`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier of the prepared message
    /// * `expiration_date` - Expiration date of the prepared message, in Unix time. Expired prepared messages can no longer be used.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<i64>>(id: T0, expiration_date: T1) -> Self {
        Self {
            id: id.into(),
            expiration_date: expiration_date.into(),
        }
    }

    /// Unique identifier of the prepared message
    #[must_use]
    pub fn id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// Expiration date of the prepared message, in Unix time. Expired prepared messages can no longer be used.
    #[must_use]
    pub fn expiration_date<T: Into<i64>>(mut self, val: T) -> Self {
        self.expiration_date = val.into();
        self
    }
}
