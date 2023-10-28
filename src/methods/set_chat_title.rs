use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to change the title of a chat. Titles can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// # Documentation
/// <https://core.telegram.org/bots/api#setchattitle>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct SetChatTitle {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// New chat title, 1-128 characters
    pub title: String,
}

impl SetChatTitle {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, title: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            title: title.into(),
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
    pub fn title(self, val: impl Into<String>) -> Self {
        Self {
            title: val.into(),
            ..self
        }
    }
}

impl TelegramMethod for SetChatTitle {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setChatTitle", self, None)
    }
}

impl AsRef<SetChatTitle> for SetChatTitle {
    fn as_ref(&self) -> &Self {
        self
    }
}
