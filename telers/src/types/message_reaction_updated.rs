use serde::{Deserialize, Serialize};
/// This object represents a change of a reaction on a message performed by a user.
/// # Documentation
/// <https://core.telegram.org/bots/api#messagereactionupdated>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageReactionUpdated {
    /// The chat containing the message the user reacted to
    pub chat: Box<crate::types::Chat>,
    /// Unique identifier of the message inside the chat
    pub message_id: i64,
    /// The user that changed the reaction, if the user isn't anonymous
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::types::User>>,
    /// The chat on behalf of which the reaction was changed, if the user is anonymous
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_chat: Option<Box<crate::types::Chat>>,
    /// Date of the change in Unix time
    pub date: i64,
    /// Previous list of reaction types that were set by the user
    pub old_reaction: Box<[crate::types::ReactionType]>,
    /// New list of reaction types that have been set by the user
    pub new_reaction: Box<[crate::types::ReactionType]>,
}
impl MessageReactionUpdated {
    /// Creates a new `MessageReactionUpdated`.
    ///
    /// # Arguments
    /// * `chat` - The chat containing the message the user reacted to
    /// * `message_id` - Unique identifier of the message inside the chat
    /// * `date` - Date of the change in Unix time
    /// * `old_reaction` - Previous list of reaction types that were set by the user
    /// * `new_reaction` - New list of reaction types that have been set by the user
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<crate::types::Chat>,
        T1: Into<i64>,
        T2: Into<i64>,
        T3Item: Into<crate::types::ReactionType>,
        T3: IntoIterator<Item = T3Item>,
        T4Item: Into<crate::types::ReactionType>,
        T4: IntoIterator<Item = T4Item>,
    >(
        chat: T0,
        message_id: T1,
        date: T2,
        old_reaction: T3,
        new_reaction: T4,
    ) -> Self {
        Self {
            chat: Box::new(chat.into()),
            message_id: message_id.into(),
            user: None,
            actor_chat: None,
            date: date.into(),
            old_reaction: old_reaction.into_iter().map(Into::into).collect(),
            new_reaction: new_reaction.into_iter().map(Into::into).collect(),
        }
    }

    /// The chat containing the message the user reacted to
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(mut self, val: T) -> Self {
        self.chat = Box::new(val.into());
        self
    }

    /// Unique identifier of the message inside the chat
    #[must_use]
    pub fn message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_id = val.into();
        self
    }

    /// The user that changed the reaction, if the user isn't anonymous
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.user = Some(Box::new(val.into()));
        self
    }

    /// The user that changed the reaction, if the user isn't anonymous
    #[must_use]
    pub fn user_option<T: Into<crate::types::User>>(mut self, val: Option<T>) -> Self {
        self.user = val.map(|val| Box::new(val.into()));
        self
    }

    /// The chat on behalf of which the reaction was changed, if the user is anonymous
    #[must_use]
    pub fn actor_chat<T: Into<crate::types::Chat>>(mut self, val: T) -> Self {
        self.actor_chat = Some(Box::new(val.into()));
        self
    }

    /// The chat on behalf of which the reaction was changed, if the user is anonymous
    #[must_use]
    pub fn actor_chat_option<T: Into<crate::types::Chat>>(mut self, val: Option<T>) -> Self {
        self.actor_chat = val.map(|val| Box::new(val.into()));
        self
    }

    /// Date of the change in Unix time
    #[must_use]
    pub fn date<T: Into<i64>>(mut self, val: T) -> Self {
        self.date = val.into();
        self
    }

    /// Previous list of reaction types that were set by the user
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn old_reactions<T: Into<Box<[crate::types::ReactionType]>>>(mut self, val: T) -> Self {
        self.old_reaction = self
            .old_reaction
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// Previous list of reaction types that were set by the user
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn old_reaction<T: Into<crate::types::ReactionType>>(mut self, val: T) -> Self {
        self.old_reaction = self
            .old_reaction
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// New list of reaction types that have been set by the user
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn new_reactions<T: Into<Box<[crate::types::ReactionType]>>>(mut self, val: T) -> Self {
        self.new_reaction = self
            .new_reaction
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// New list of reaction types that have been set by the user
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn new_reaction<T: Into<crate::types::ReactionType>>(mut self, val: T) -> Self {
        self.new_reaction = self
            .new_reaction
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }
}
