use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, ChatMember},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to get information about a member of a chat. The method is guaranteed to work for other users, only if the bot is an administrator in the chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#getchatmember>
/// # Returns
/// Returns a [`ChatMember`] object on success.
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct GetChatMember {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
}

impl GetChatMember {
    #[must_use]
    pub fn new<T: Into<ChatIdKind>>(chat_id: T, user_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    #[must_use]
    pub fn user_id(mut self, val: i64) -> Self {
        self.user_id = val;
        self
    }
}

impl TelegramMethod for GetChatMember {
    type Method = Self;
    type Return = ChatMember;

    fn build_request(&self, _bot: &Bot) -> Request<Self::Method> {
        Request::new("getChatMember", self, None)
    }
}
