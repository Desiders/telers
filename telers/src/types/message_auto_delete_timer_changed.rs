use serde::{Deserialize, Serialize};
/// This object represents a service message about a change in auto-delete timer settings.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageautodeletetimerchanged>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageAutoDeleteTimerChanged {
    /// New auto-delete time for messages in the chat; in seconds
    pub message_auto_delete_time: i64,
}
impl MessageAutoDeleteTimerChanged {
    /// Creates a new `MessageAutoDeleteTimerChanged`.
    ///
    /// # Arguments
    /// * `message_auto_delete_time` - New auto-delete time for messages in the chat; in seconds
    #[must_use]
    pub fn new<T0: Into<i64>>(message_auto_delete_time: T0) -> Self {
        Self {
            message_auto_delete_time: message_auto_delete_time.into(),
        }
    }

    /// New auto-delete time for messages in the chat; in seconds
    #[must_use]
    pub fn message_auto_delete_time<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_auto_delete_time = val.into();
        self
    }
}
