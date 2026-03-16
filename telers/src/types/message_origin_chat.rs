use serde::{Deserialize, Serialize};
/// The message was originally sent on behalf of a chat to a group chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageoriginchat>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageOriginChat {
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// Chat that sent the message originally
    pub sender_chat: Box<crate::types::Chat>,
    /// For messages originally sent by an anonymous chat administrator, original message author signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<Box<str>>,
}
impl MessageOriginChat {
    /// Creates a new `MessageOriginChat`.
    ///
    /// # Arguments
    /// * `date` - Date the message was sent originally in Unix time
    /// * `sender_chat` - Chat that sent the message originally
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::Chat>>(date: T0, sender_chat: T1) -> Self {
        Self {
            date: date.into(),
            sender_chat: Box::new(sender_chat.into()),
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

    /// Chat that sent the message originally
    #[must_use]
    pub fn sender_chat<T: Into<crate::types::Chat>>(self, val: T) -> Self {
        let mut this = self;
        this.sender_chat = Box::new(val.into());
        this
    }

    /// For messages originally sent by an anonymous chat administrator, original message author signature
    #[must_use]
    pub fn author_signature<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.author_signature = Some(val.into());
        this
    }

    /// For messages originally sent by an anonymous chat administrator, original message author signature
    #[must_use]
    pub fn author_signature_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.author_signature = val.map(Into::into);
        this
    }
}
