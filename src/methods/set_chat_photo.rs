use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// # Documentation
/// <https://core.telegram.org/bots/api#setchatphoto>
/// # Returns
/// Returns `true` on success
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct SetChatPhoto<'a> {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// New chat photo, uploaded using `multipart/form-data`
    pub photo: InputFile<'a>,
}

impl<'a> SetChatPhoto<'a> {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, photo: impl Into<InputFile<'a>>) -> Self {
        Self {
            chat_id: chat_id.into(),
            photo: photo.into(),
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
    pub fn photo(self, val: impl Into<InputFile<'a>>) -> Self {
        Self {
            photo: val.into(),
            ..self
        }
    }
}

impl TelegramMethod for SetChatPhoto {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setChatPhoto", self, None)
    }
}

impl AsRef<SetChatPhoto<'_>> for SetChatPhoto<'_> {
    fn as_ref(&self) -> &Self {
        self
    }
}
