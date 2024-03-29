use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;

/// Use this method to reopen a closed topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the `can_manage_topics` administrator rights, unless it is the creator of the topic.
/// # Documentation
/// <https://core.telegram.org/bots/api#reopenforumtopic>
/// # Returns
/// Returns `true` on success
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct ReopenForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
}

impl ReopenForumTopic {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, message_thread_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id,
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn message_thread_id(self, val: i64) -> Self {
        Self {
            message_thread_id: val,
            ..self
        }
    }
}

impl TelegramMethod for ReopenForumTopic {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("reopenForumTopic", self, None)
    }
}

impl AsRef<ReopenForumTopic> for ReopenForumTopic {
    fn as_ref(&self) -> &Self {
        self
    }
}
