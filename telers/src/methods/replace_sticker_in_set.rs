use super::base::{prepare_input_sticker, Request, TelegramMethod};

use crate::{client::Bot, types::InputSticker};

use serde::Serialize;

/// Use this method to replace an existing sticker in a sticker set with a new one. The method is equivalent to calling [`DeleteStickerFromSet`](crate::methods::DeleteStickerFromSet), then [`AddStickerToSet`](crate::methods::AddStickerToSet), then [`SetStickerPositionInSet`](crate::methods::SetStickerPositionInSet).
/// # Documentation
/// <https://core.telegram.org/bots/api#replacestickerinset>
/// # Returns
/// On success, `true` is returned
#[derive(Debug, PartialEq, Serialize)]
pub struct ReplaceStickerInSet {
    /// User identifier of the sticker set owner
    pub user_id: i64,
    /// Sticker set name
    pub name: String,
    /// File identifier of the replaced sticker
    pub old_sticker: String,
    /// A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set remains unchanged.
    pub sticker: InputSticker,
}

impl ReplaceStickerInSet {
    #[must_use]
    pub fn new(
        user_id: i64,
        name: impl Into<String>,
        old_sticker: impl Into<String>,
        sticker: impl Into<InputSticker>,
    ) -> Self {
        Self {
            user_id,
            name: name.into(),
            old_sticker: old_sticker.into(),
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
    pub fn old_sticker(self, val: impl Into<String>) -> Self {
        Self {
            old_sticker: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn sticker(self, val: impl Into<InputSticker>) -> Self {
        Self {
            sticker: val.into(),
            ..self
        }
    }
}

impl TelegramMethod for ReplaceStickerInSet {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(mut self, _bot: &Bot<Client>) -> Request<Self::Method> {
        let mut files = vec![];
        prepare_input_sticker(&mut files, &mut self.sticker);

        Request::new("replaceStickerInSet", self, Some(files))
    }
}

impl AsRef<ReplaceStickerInSet> for ReplaceStickerInSet {
    fn as_ref(&self) -> &Self {
        self
    }
}
