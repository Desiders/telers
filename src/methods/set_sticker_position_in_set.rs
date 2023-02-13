use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to move a sticker in a set created by the bot to a specific position
/// # Documentation
/// <https://core.telegram.org/bots/api#setstickerpositioninset>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SetStickerPositionInSet {
    /// File identifier of the sticker
    pub sticker: String,
    /// New sticker position in the set, zero-based
    pub position: i64,
}

impl SetStickerPositionInSet {
    #[must_use]
    pub fn new<T: Into<String>>(sticker: T, position: i64) -> Self {
        Self {
            sticker: sticker.into(),
            position,
        }
    }

    #[must_use]
    pub fn sticker<T: Into<String>>(mut self, val: T) -> Self {
        self.sticker = val.into();
        self
    }

    #[must_use]
    pub fn position(mut self, val: i64) -> Self {
        self.position = val;
        self
    }
}

impl TelegramMethod for SetStickerPositionInSet {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setStickerPositionInSet", self, None)
    }
}