use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;

/// Use this method to move a sticker in a set created by the bot to a specific position
/// # Documentation
/// <https://core.telegram.org/bots/api#setstickerpositioninset>
/// # Returns
/// Returns `true` on success
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct SetStickerPositionInSet {
    /// File identifier of the sticker
    pub sticker: String,
    /// New sticker position in the set, zero-based
    pub position: i64,
}

impl SetStickerPositionInSet {
    #[must_use]
    pub fn new(sticker: impl Into<String>, position: i64) -> Self {
        Self {
            sticker: sticker.into(),
            position,
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
    pub fn position(self, val: i64) -> Self {
        Self {
            position: val,
            ..self
        }
    }
}

impl TelegramMethod for SetStickerPositionInSet {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setStickerPositionInSet", self, None)
    }
}

impl AsRef<SetStickerPositionInSet> for SetStickerPositionInSet {
    fn as_ref(&self) -> &Self {
        self
    }
}
