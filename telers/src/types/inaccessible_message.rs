use serde::{Deserialize, Serialize};
/// This object describes a message that was deleted or is otherwise inaccessible to the bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#inaccessiblemessage>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InaccessibleMessage {
    /// Chat the message belonged to
    pub chat: Box<crate::types::Chat>,
    /// Unique message identifier inside the chat
    pub message_id: i64,
    /// Always 0. The field can be used to differentiate regular and inaccessible messages.
    pub date: i64,
}
impl InaccessibleMessage {
    /// Creates a new `InaccessibleMessage`.
    ///
    /// # Arguments
    /// * `chat` - Chat the message belonged to
    /// * `message_id` - Unique message identifier inside the chat
    /// * `date` - Always 0. The field can be used to differentiate regular and inaccessible messages.
    #[must_use]
    pub fn new<T0: Into<crate::types::Chat>, T1: Into<i64>, T2: Into<i64>>(
        chat: T0,
        message_id: T1,
        date: T2,
    ) -> Self {
        Self {
            chat: Box::new(chat.into()),
            message_id: message_id.into(),
            date: date.into(),
        }
    }

    /// Chat the message belonged to
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

    /// Always 0. The field can be used to differentiate regular and inaccessible messages.
    #[must_use]
    pub fn date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.date = val.into();
        this
    }
}
