use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, ChatMember},
};

use serde::Serialize;

/// Use this method to get information about a member of a chat. The method is only guaranteed to work for other users if the bot is an administrator in the chat
/// # Documentation
/// <https://core.telegram.org/bots/api#getchatmember>
/// # Returns
/// Returns a [`ChatMember`] object on success
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct GetChatMember {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
}

impl GetChatMember {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, user_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
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
    pub fn user_id(self, val: i64) -> Self {
        Self {
            user_id: val,
            ..self
        }
    }
}

impl TelegramMethod for GetChatMember {
    type Method = Self;
    type Return = ChatMember;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("getChatMember", self, None)
    }
}

impl AsRef<GetChatMember> for GetChatMember {
    fn as_ref(&self) -> &Self {
        self
    }
}
