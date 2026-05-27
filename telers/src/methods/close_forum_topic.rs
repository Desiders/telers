use crate::client::Bot;
use serde::Serialize;
/// Use this method to close an open topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the `can_manage_topics` administrator rights, unless it is the creator of the topic. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#closeforumtopic>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct CloseForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
}
impl CloseForumTopic {
    /// Creates a new `CloseForumTopic`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup in the format @username
    /// * `message_thread_id` - Unique identifier for the target message thread of the forum topic
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<i64>>(
        chat_id: T0,
        message_thread_id: T1,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: message_thread_id.into(),
        }
    }

    /// Unique identifier for the target chat or username of the target supergroup in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    /// Unique identifier for the target message thread of the forum topic
    #[must_use]
    pub fn message_thread_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_thread_id = val.into();
        self
    }
}
impl super::TelegramMethod for CloseForumTopic {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("closeForumTopic", self, None)
    }
}
