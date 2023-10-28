use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{Chat, ChatIdKind},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to get up to date information about the chat (current name of the user for one-on-one conversations, current username of a user, group or channel, etc.).
/// # Documentation
/// <https://core.telegram.org/bots/api#getchat>
/// # Returns
/// Returns a [`Chat`] object on success
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct GetChat {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
}

impl GetChat {
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

impl TelegramMethod for GetChat {
    type Method = Self;
    type Return = Chat;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("getChat", self, None)
    }
}

impl AsRef<GetChat> for GetChat {
    fn as_ref(&self) -> &Self {
        self
    }
}
