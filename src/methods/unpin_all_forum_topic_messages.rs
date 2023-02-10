use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to clear the list of pinned messages in a forum topic. The bot must be an administrator in the chat for this to work and must have the `can_pin_messages` administrator right in the supergroup.
/// # Documentation
/// <https://core.telegram.org/bots/api#unpinallforumtopicmessages>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct UnpinAllForumTopicMessages {
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
}

impl UnpinAllForumTopicMessages {
    #[must_use]
    pub fn new<T: Into<ChatIdKind>>(chat_id: T, message_thread_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id,
        }
    }

    #[must_use]
    pub fn chat_id<C: Into<ChatIdKind>>(mut self, val: C) -> Self {
        self.chat_id = val.into();
        self
    }

    #[must_use]
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = val;
        self
    }
}

impl TelegramMethod for UnpinAllForumTopicMessages {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("unpinAllForumTopicMessages", self, None)
    }
}
