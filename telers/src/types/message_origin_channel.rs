use serde::{Deserialize, Serialize};
/// The message was originally sent to a channel chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageoriginchannel>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageOriginChannel {
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// Channel chat to which the message was originally sent
    pub chat: Box<crate::types::Chat>,
    /// Unique message identifier inside the chat
    pub message_id: i64,
    /// Signature of the original post author
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<Box<str>>,
}
impl MessageOriginChannel {
    /// Creates a new `MessageOriginChannel`.
    ///
    /// # Arguments
    /// * `date` - Date the message was sent originally in Unix time
    /// * `chat` - Channel chat to which the message was originally sent
    /// * `message_id` - Unique message identifier inside the chat
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::Chat>, T2: Into<i64>>(
        date: T0,
        chat: T1,
        message_id: T2,
    ) -> Self {
        Self {
            date: date.into(),
            chat: Box::new(chat.into()),
            message_id: message_id.into(),
            author_signature: None,
        }
    }

    /// Date the message was sent originally in Unix time
    #[must_use]
    pub fn date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.date = val.into();
        this
    }

    /// Channel chat to which the message was originally sent
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(self, val: T) -> Self {
        let mut this = self;
        this.chat = Box::new(val.into());
        this
    }

    /// Unique message identifier inside the chat
    #[must_use]
    pub fn message_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.message_id = val.into();
        this
    }

    /// Signature of the original post author
    #[must_use]
    pub fn author_signature<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.author_signature = Some(val.into());
        this
    }

    /// Signature of the original post author
    #[must_use]
    pub fn author_signature_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.author_signature = val.map(Into::into);
        this
    }
}
