use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to delete a sticker set that was created by the bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#deletestickerset>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct DeleteStickerSet {
    /// Sticker set name
    pub name: String,
}

impl DeleteStickerSet {
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    #[must_use]
    pub fn name(self, val: impl Into<String>) -> Self {
        Self { name: val.into() }
    }
}

impl TelegramMethod for DeleteStickerSet {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("deleteStickerSet", self, None)
    }
}
