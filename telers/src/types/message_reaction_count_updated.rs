use serde::{Deserialize, Serialize};
/// This object represents reaction changes on a message with anonymous reactions.
/// # Documentation
/// <https://core.telegram.org/bots/api#messagereactioncountupdated>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageReactionCountUpdated {
    /// The chat containing the message
    pub chat: Box<crate::types::Chat>,
    /// Unique message identifier inside the chat
    pub message_id: i64,
    /// Date of the change in Unix time
    pub date: i64,
    /// List of reactions that are present on the message
    pub reactions: Box<[crate::types::ReactionCount]>,
}
impl MessageReactionCountUpdated {
    /// Creates a new `MessageReactionCountUpdated`.
    ///
    /// # Arguments
    /// * `chat` - The chat containing the message
    /// * `message_id` - Unique message identifier inside the chat
    /// * `date` - Date of the change in Unix time
    /// * `reactions` - List of reactions that are present on the message
    #[must_use]
    pub fn new<
        T0: Into<crate::types::Chat>,
        T1: Into<i64>,
        T2: Into<i64>,
        T3Item: Into<crate::types::ReactionCount>,
        T3: IntoIterator<Item = T3Item>,
    >(
        chat: T0,
        message_id: T1,
        date: T2,
        reactions: T3,
    ) -> Self {
        Self {
            chat: Box::new(chat.into()),
            message_id: message_id.into(),
            date: date.into(),
            reactions: reactions.into_iter().map(Into::into).collect(),
        }
    }

    /// The chat containing the message
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(mut self, val: T) -> Self {
        self.chat = Box::new(val.into());
        self
    }

    /// Unique message identifier inside the chat
    #[must_use]
    pub fn message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_id = val.into();
        self
    }

    /// Date of the change in Unix time
    #[must_use]
    pub fn date<T: Into<i64>>(mut self, val: T) -> Self {
        self.date = val.into();
        self
    }

    /// List of reactions that are present on the message
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn reactions<T: Into<Box<[crate::types::ReactionCount]>>>(mut self, val: T) -> Self {
        self.reactions = self
            .reactions
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// List of reactions that are present on the message
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn reaction<T: Into<crate::types::ReactionCount>>(mut self, val: T) -> Self {
        self.reactions = self
            .reactions
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }
}
