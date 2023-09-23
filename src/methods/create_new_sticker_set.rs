use super::base::{prepare_input_stickers, Request, TelegramMethod};

use crate::{client::Bot, types::InputSticker};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created.
/// # Documentation
/// <https://core.telegram.org/bots/api#createnewstickerset>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateNewStickerSet<'a> {
    /// User identifier of created sticker set owner
    pub user_id: i64,
    /// Short name of sticker set, to be used in `t.me/addstickers/` URLs (e.g., animals). Can contain only english letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in `_by_<bot username>`. `<bot_username>` is case insensitive. 1-64 characters.
    pub name: String,
    /// Sticker set title, 1-64 characters
    pub title: String,
    /// A JSON-serialized list of 1-50 initial stickers to be added to the sticker set
    pub stickers: Vec<InputSticker<'a>>,
    /// Format of stickers in the set, must be one of `static`, `animated`, `video`
    pub sticker_format: String,
    /// Type of stickers in the set, pass `regular`, `mask` or `custom_emoji`. By default, a regular sticker set is created.
    pub sticker_type: Option<String>,
    /// Pass `True` if stickers in the sticker set must be repainted to the color of text when used in messages, the accent color if used as emoji status, white on chat photos, or another appropriate color based on context; for custom emoji sticker sets only
    pub needs_repainting: Option<bool>,
}

impl<'a> CreateNewStickerSet<'a> {
    #[must_use]
    pub fn new(
        user_id: i64,
        name: impl Into<String>,
        title: impl Into<String>,
        stickers: impl IntoIterator<Item = InputSticker<'a>>,
        sticker_format: impl Into<String>,
    ) -> Self {
        Self {
            user_id,
            name: name.into(),
            title: title.into(),
            stickers: stickers.into_iter().collect(),
            sticker_format: sticker_format.into(),
            sticker_type: None,
            needs_repainting: None,
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
    pub fn title(self, val: impl Into<String>) -> Self {
        Self {
            title: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn sticker(self, val: InputSticker<'a>) -> Self {
        Self {
            stickers: self.stickers.into_iter().chain(Some(val)).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn stickers(self, val: impl IntoIterator<Item = InputSticker<'a>>) -> Self {
        Self {
            stickers: self.stickers.into_iter().chain(val).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn sticker_format(self, val: impl Into<String>) -> Self {
        Self {
            sticker_format: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn sticker_type(self, val: impl Into<String>) -> Self {
        Self {
            sticker_type: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn needs_repainting(self, val: bool) -> Self {
        Self {
            needs_repainting: Some(val),
            ..self
        }
    }
}

impl<'a> CreateNewStickerSet<'a> {
    #[must_use]
    pub fn sticker_type_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            sticker_type: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn needs_repainting_option(self, val: Option<bool>) -> Self {
        Self {
            needs_repainting: val,
            ..self
        }
    }
}

impl<'a> TelegramMethod for CreateNewStickerSet<'a> {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        let mut files = vec![];
        prepare_input_stickers(&mut files, &self.stickers);

        Request::new("createNewStickerSet", self, Some(files))
    }
}
