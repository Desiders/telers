use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;

/// Use this method to remove a message from the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the `can_pin_messages` administrator right in a supergroup or `can_edit_messages` administrator right in a channel.
/// # Documentation
/// <https://core.telegram.org/bots/api#unpinchatmessage>
/// # Returns
/// Returns `true` on success
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct UnpinChatMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Identifier of a message to unpin. If not specified, the most recent pinned message (by sending date) will be unpinned.
    pub message_id: i64,
}

impl UnpinChatMessage {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, message_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id,
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
    pub fn message_id(self, val: i64) -> Self {
        Self {
            message_id: val,
            ..self
        }
    }
}

impl TelegramMethod for UnpinChatMessage {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("unpinChatMessage", self, None)
    }
}

impl AsRef<UnpinChatMessage> for UnpinChatMessage {
    fn as_ref(&self) -> &Self {
        self
    }
}
