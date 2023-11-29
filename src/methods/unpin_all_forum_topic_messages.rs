use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to clear the list of pinned messages in a forum topic. The bot must be an administrator in the chat for this to work and must have the `can_pin_messages` administrator right in the supergroup.
/// # Documentation
/// <https://core.telegram.org/bots/api#unpinallforumtopicmessages>
/// # Returns
/// Returns `true` on success
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct UnpinAllForumTopicMessages {
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
}

impl UnpinAllForumTopicMessages {
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

impl TelegramMethod for UnpinAllForumTopicMessages {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("unpinAllForumTopicMessages", self, None)
    }
}

impl AsRef<UnpinAllForumTopicMessages> for UnpinAllForumTopicMessages {
    fn as_ref(&self) -> &Self {
        self
    }
}
