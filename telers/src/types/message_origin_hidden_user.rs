use serde::{Deserialize, Serialize};
/// The message was originally sent by an unknown user.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageoriginhiddenuser>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageOriginHiddenUser {
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// Name of the user that sent the message originally
    pub sender_user_name: Box<str>,
}
impl MessageOriginHiddenUser {
    /// Creates a new `MessageOriginHiddenUser`.
    ///
    /// # Arguments
    /// * `date` - Date the message was sent originally in Unix time
    /// * `sender_user_name` - Name of the user that sent the message originally
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<Box<str>>>(date: T0, sender_user_name: T1) -> Self {
        Self {
            date: date.into(),
            sender_user_name: sender_user_name.into(),
        }
    }

    /// Date the message was sent originally in Unix time
    #[must_use]
    pub fn date<T: Into<i64>>(mut self, val: T) -> Self {
        self.date = val.into();
        self
    }

    /// Name of the user that sent the message originally
    #[must_use]
    pub fn sender_user_name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.sender_user_name = val.into();
        self
    }
}
