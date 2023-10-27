use super::base::{prepare_input_sticker, Request, TelegramMethod};

use crate::{client::Bot, types::InputSticker};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to add a new sticker to a set created by the bot.
/// The format of the added sticker must match the format of the other stickers in the set.
/// Emoji sticker sets can have up to 200 stickers.
/// Animated and video sticker sets can have up to 50 stickers.
/// Static sticker sets can have up to 120 stickers.
/// # Documentation
/// <https://core.telegram.org/bots/api#addstickertoset>
/// # Returns
/// `True` on success
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AddStickerToSet<'a> {
    /// User identifier of sticker set owner
    pub user_id: i64,
    /// Sticker set name
    pub name: String,
    /// A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set isn't changed.
    pub sticker: InputSticker<'a>,
}

impl<'a> AddStickerToSet<'a> {
    #[must_use]
    pub fn new(
        user_id: i64,
        name: impl Into<String>,
        sticker: impl Into<InputSticker<'a>>,
    ) -> Self {
        Self {
            user_id,
            name: name.into(),
            sticker: sticker.into(),
        }
    }

    #[must_use]
    pub fn user_id(self, val: i64) -> Self {
        Self {
            user_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn name(self, val: impl Into<String>) -> Self {
        Self {
            name: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn sticker(self, val: impl Into<InputSticker<'a>>) -> Self {
        Self {
            sticker: val.into(),
            ..self
        }
    }
}

impl<'a> TelegramMethod for AddStickerToSet<'a> {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        let mut files = vec![];
        prepare_input_sticker(&mut files, &self.sticker);

        Request::new("addStickerToSet", self, Some(files.into()))
    }
}

impl<'a> AsRef<AddStickerToSet<'a>> for AddStickerToSet<'a> {
    fn as_ref(&self) -> &Self {
        self
    }
}
