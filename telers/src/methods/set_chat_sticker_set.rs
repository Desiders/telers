use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;

/// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field `can_set_sticker_set` optionally returned in [`GetChat`](crate::methods::GetChat) requests to check if the bot can use this method.
/// # Documentation
/// <https://core.telegram.org/bots/api#setchatstickerset>
/// # Returns
/// Returns `true` on success
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct SetChatStickerSet {
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Name of the sticker set to be set as the group sticker set
    pub sticker_set_name: String,
}

impl SetChatStickerSet {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, sticker_set_name: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            sticker_set_name: sticker_set_name.into(),
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
    pub fn sticker_set_name(self, val: impl Into<String>) -> Self {
        Self {
            sticker_set_name: val.into(),
            ..self
        }
    }
}

impl TelegramMethod for SetChatStickerSet {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setChatStickerSet", self, None)
    }
}

impl AsRef<SetChatStickerSet> for SetChatStickerSet {
    fn as_ref(&self) -> &Self {
        self
    }
}
