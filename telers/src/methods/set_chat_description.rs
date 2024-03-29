use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;

/// Use this method to change the description of a group, a supergroup or a channel. descriptions can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// # Documentation
/// <https://core.telegram.org/bots/api#setchatdescription>
/// # Returns
/// Returns `true` on success
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct SetChatDescription {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// New chat description, 0-255 characters
    pub description: String,
}

impl SetChatDescription {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, description: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            description: description.into(),
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
    pub fn description(self, val: impl Into<String>) -> Self {
        Self {
            description: val.into(),
            ..self
        }
    }
}

impl TelegramMethod for SetChatDescription {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setChatDescription", self, None)
    }
}

impl AsRef<SetChatDescription> for SetChatDescription {
    fn as_ref(&self) -> &Self {
        self
    }
}
