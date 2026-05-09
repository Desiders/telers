use crate::client::Bot;
use serde::Serialize;
/// Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the `can_manage_topics` administrator rights, unless it is the creator of the topic. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#reopenforumtopic>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct ReopenForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
}
impl ReopenForumTopic {
    /// Creates a new `ReopenForumTopic`.
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
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// Unique identifier for the target message thread of the forum topic
    #[must_use]
    pub fn message_thread_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.message_thread_id = val.into();
        this
    }
}
impl super::TelegramMethod for ReopenForumTopic {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("reopenForumTopic", self, None)
    }
}
