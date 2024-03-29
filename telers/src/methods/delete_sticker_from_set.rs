use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;

/// Use this method to delete a sticker from a set created by the bot
/// # Documentation
/// <https://core.telegram.org/bots/api#deletestickerfromset>
/// # Returns
/// Returns `true` on success
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct DeleteStickerFromSet {
    /// File identifier of the sticker
    pub sticker: String,
}

impl DeleteStickerFromSet {
    #[must_use]
    pub fn new(sticker: impl Into<String>) -> Self {
        Self {
            sticker: sticker.into(),
        }
    }

    #[must_use]
    pub fn sticker(self, val: impl Into<String>) -> Self {
        Self {
            sticker: val.into(),
        }
    }
}

impl TelegramMethod for DeleteStickerFromSet {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("deleteStickerFromSet", self, None)
    }
}

impl AsRef<DeleteStickerFromSet> for DeleteStickerFromSet {
    fn as_ref(&self) -> &Self {
        self
    }
}
