use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::MaskPosition};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to change the [`MaskPosition`] of a mask sticker. The sticker must belong to a sticker set that was created by the bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#setstickermaskposition>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SetStickerMaskPosition {
    /// File identifier of the sticker
    pub sticker: String,
    /// A JSON-serialized object with the position where the mask should be placed on faces. Omit the parameter to remove the mask position.
    pub mask_position: Option<MaskPosition>,
}

impl SetStickerMaskPosition {
    #[must_use]
    pub fn new(sticker: impl Into<String>) -> Self {
        Self {
            sticker: sticker.into(),
            mask_position: None,
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
    pub fn mask_position(self, val: MaskPosition) -> Self {
        Self {
            mask_position: Some(val),
            ..self
        }
    }
}

impl TelegramMethod for SetStickerMaskPosition {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setStickerMaskPosition", self, None)
    }
}
