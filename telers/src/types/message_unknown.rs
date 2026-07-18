use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// This object represents a Message unknown to this version of the library.
/// # Notes
/// Fields shared by all known variants are parsed as usual; everything else is kept in `extra`, so the object can be inspected and reserialized without data loss.
/// # Documentation
/// <https://core.telegram.org/bots/api#message>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageUnknown {
    /// Unique message identifier inside this chat; 0 for ephemeral messages. In specific instances (e.g., a message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent.
    pub message_id: i64,
    /// Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    pub date: i64,
    /// Chat the message belongs to
    pub chat: Box<crate::types::Chat>,
    #[serde(flatten)]
    pub extra: BTreeMap<Box<str>, serde_json::Value>,
}
impl MessageUnknown {
    /// Creates a new `MessageUnknown`.
    ///
    /// # Arguments
    /// * `message_id` - Unique message identifier inside this chat; 0 for ephemeral messages. In specific instances (e.g., a message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent.
    /// * `date` - Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// * `chat` - Chat the message belongs to
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>, T2: Into<crate::types::Chat>>(
        message_id: T0,
        date: T1,
        chat: T2,
    ) -> Self {
        Self {
            message_id: message_id.into(),
            date: date.into(),
            chat: Box::new(chat.into()),
            extra: BTreeMap::new(),
        }
    }

    /// Unique message identifier inside this chat; 0 for ephemeral messages. In specific instances (e.g., a message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent.
    #[must_use]
    pub fn message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_id = val.into();
        self
    }

    /// Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    #[must_use]
    pub fn date<T: Into<i64>>(mut self, val: T) -> Self {
        self.date = val.into();
        self
    }

    /// Chat the message belongs to
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(mut self, val: T) -> Self {
        self.chat = Box::new(val.into());
        self
    }
}
