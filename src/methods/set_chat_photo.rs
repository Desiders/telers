use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// # Documentation
/// <https://core.telegram.org/bots/api#setchatphoto>
/// # Returns
/// Returns `True` on success.
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SetChatPhoto<'a> {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// New chat photo, uploaded using `multipart/form-data`
    pub photo: InputFile<'a>,
}

impl<'a> SetChatPhoto<'a> {
    #[must_use]
    pub fn new<C: Into<ChatIdKind>, P: Into<InputFile<'a>>>(chat_id: C, photo: P) -> Self {
        Self {
            chat_id: chat_id.into(),
            photo: photo.into(),
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    #[must_use]
    pub fn photo<P: Into<InputFile<'a>>>(mut self, val: P) -> Self {
        self.photo = val.into();
        self
    }
}

impl TelegramMethod for SetChatPhoto {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setChatPhoto", self, None)
    }
}
