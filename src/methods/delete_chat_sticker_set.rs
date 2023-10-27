use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to delete a group sticker set from a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field `can_set_sticker_set` optionally returned in [`GetChat`](crate::methods::GetChat) requests to check if the bot can use this method.
/// # Documentation
/// <https://core.telegram.org/bots/api#deletechatstickerset>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct DeleteChatStickerSet {
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
}

impl DeleteChatStickerSet {
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

impl TelegramMethod for DeleteChatStickerSet {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("deleteChatStickerSet", self, None)
    }
}

impl AsRef<DeleteChatStickerSet> for DeleteChatStickerSet {
    fn as_ref(&self) -> &Self {
        self
    }
}
