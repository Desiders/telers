use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#setstickeremojilist>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SetStickerEmojiList {
    /// File identifier of the sticker
    pub sticker: String,
    /// A JSON-serialized list of 1-20 emoji associated with the sticker
    pub emoji_list: Vec<String>,
}

impl SetStickerEmojiList {
    #[must_use]
    pub fn new(sticker: impl Into<String>) -> Self {
        Self {
            sticker: sticker.into(),
            emoji_list: vec![],
        }
    }

    #[must_use]
    pub fn sticker(self, val: impl Into<String>) -> Self {
        Self {
            sticker: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn emoji(self, val: impl Into<String>) -> Self {
        Self {
            emoji_list: self
                .emoji_list
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn emoji_list<T, I>(self, val: I) -> Self
    where
        T: Into<String>,
        I: IntoIterator<Item = T>,
    {
        Self {
            emoji_list: self
                .emoji_list
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
            ..self
        }
    }
}

impl TelegramMethod for SetStickerEmojiList {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setStickerEmojiList", self, None)
    }
}

impl AsRef<SetStickerEmojiList> for SetStickerEmojiList {
    fn as_ref(&self) -> &Self {
        self
    }
}
