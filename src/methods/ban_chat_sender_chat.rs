use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is [`unbanned`](crate::methods::UnbanChatSenderChat), the owner of the banned chat won't be able to send messages on behalf of **any of their channels**. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights.
/// # Documentation
/// <https://core.telegram.org/bots/api#banchatsenderchat>
/// # Returns
/// Returns `true` on success
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct BanChatSenderChat {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier of the target sender chat
    pub sender_chat_id: i64,
}

impl BanChatSenderChat {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, sender_chat_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            sender_chat_id,
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
    pub fn sender_chat_id(self, val: i64) -> Self {
        Self {
            sender_chat_id: val,
            ..self
        }
    }
}

impl TelegramMethod for BanChatSenderChat {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("banChatSenderChat", self, None)
    }
}

impl AsRef<BanChatSenderChat> for BanChatSenderChat {
    fn as_ref(&self) -> &Self {
        self
    }
}
