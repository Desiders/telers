use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, ChatMember},
};

use serde::Serialize;

/// Use this method to get a list of administrators in a chat, which aren't bots.
/// # Documentation
/// <https://core.telegram.org/bots/api#getchatadministrators>
/// # Returns
/// Returns an Array of [`ChatMember`] objects
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct GetChatAdministrators {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
}

impl GetChatAdministrators {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: val.into(),
        }
    }
}

impl TelegramMethod for GetChatAdministrators {
    type Method = Self;
    type Return = Vec<ChatMember>;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("getChatAdministrators", self, None)
    }
}

impl AsRef<GetChatAdministrators> for GetChatAdministrators {
    fn as_ref(&self) -> &Self {
        self
    }
}
