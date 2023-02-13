use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::StickerSet};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to get a sticker set.
/// # Documentation
/// <https://core.telegram.org/bots/api#getstickerset>
/// # Returns
/// On success, a [`StickerSet`] object is returned
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct GetStickerSet {
    /// Name of the sticker set
    pub name: String,
}

impl GetStickerSet {
    #[must_use]
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self { name: name.into() }
    }

    #[must_use]
    pub fn name<T: Into<String>>(mut self, val: T) -> Self {
        self.name = val.into();
        self
    }
}

impl TelegramMethod for GetStickerSet {
    type Method = Self;
    type Return = StickerSet;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("getStickerSet", self, None)
    }
}