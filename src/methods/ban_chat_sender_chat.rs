use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is [`unbanned`](crate::methods::UnbanChatSenderChat), the owner of the banned chat won't be able to send messages on behalf of **any of their channels**. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights.
/// # Documentation
/// <https://core.telegram.org/bots/api#banchatsenderchat>
/// # Returns
/// Returns `True` on success.
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct BanChatSenderChat {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier of the target sender chat
    pub sender_chat_id: i64,
}

impl BanChatSenderChat {
    #[must_use]
    pub fn new<T: Into<ChatIdKind>>(chat_id: T, sender_chat_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            sender_chat_id,
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    #[must_use]
    pub fn sender_chat_id(mut self, val: i64) -> Self {
        self.sender_chat_id = val;
        self
    }
}

impl TelegramMethod for BanChatSenderChat {
    type Method = Self;
    type Return = bool;

    fn build_request(&self, _bot: &Bot) -> Request<Self::Method> {
        Request::new("banChatSenderChat", self, None)
    }
}
