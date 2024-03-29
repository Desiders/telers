use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;

/// Use this method to unban a previously banned channel chat in a supergroup or channel. The bot must be an administrator for this to work and must have the appropriate administrator rights.
/// # Documentation
/// <https://core.telegram.org/bots/api#unbanchatsenderchat>
/// # Returns
/// Returns `true` on success
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct UnbanChatSenderChat {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier of the target sender chat
    pub sender_chat_id: i64,
}

impl UnbanChatSenderChat {
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

impl TelegramMethod for UnbanChatSenderChat {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("unbanChatSenderChat", self, None)
    }
}

impl AsRef<UnbanChatSenderChat> for UnbanChatSenderChat {
    fn as_ref(&self) -> &Self {
        self
    }
}
