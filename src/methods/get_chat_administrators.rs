use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, ChatMember},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to get a list of administrators in a chat, which aren't bots.
/// # Documentation
/// <https://core.telegram.org/bots/api#getchatadministrators>
/// # Returns
/// Returns an Array of [`ChatMember`] objects.
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct GetChatAdministrators {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
}

impl GetChatAdministrators {
    #[must_use]
    pub fn new<T: Into<ChatIdKind>>(chat_id: T) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }
}

impl TelegramMethod for GetChatAdministrators {
    type Method = Self;
    type Return = Vec<ChatMember>;

    fn build_request(&self, _: &Bot) -> Request<Self::Method> {
        Request::new("getChatAdministrators", self, None)
    }
}