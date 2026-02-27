use serde::{Deserialize, Serialize};
/// This object represents a unique message identifier.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageid>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageId {
    /// Unique message identifier. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    pub message_id: i64,
}
impl MessageId {
    /// Creates a new `MessageId`.
    ///
    /// # Arguments
    /// * `message_id` - Unique message identifier. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    #[must_use]
    pub fn new<T0: Into<i64>>(message_id: T0) -> Self {
        Self {
            message_id: message_id.into(),
        }
    }

    /// Unique message identifier. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    #[must_use]
    pub fn message_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.message_id = val.into();
        this
    }
}
