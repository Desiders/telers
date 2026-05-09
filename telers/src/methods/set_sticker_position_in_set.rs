use crate::client::Bot;
use serde::Serialize;
/// Use this method to move a sticker in a set created by the bot to a specific position. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setstickerpositioninset>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetStickerPositionInSet {
    /// File identifier of the sticker
    pub sticker: Box<str>,
    /// New sticker position in the set, zero-based
    pub position: i64,
}
impl SetStickerPositionInSet {
    /// Creates a new `SetStickerPositionInSet`.
    ///
    /// # Arguments
    /// * `sticker` - File identifier of the sticker
    /// * `position` - New sticker position in the set, zero-based
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<i64>>(sticker: T0, position: T1) -> Self {
        Self {
            sticker: sticker.into(),
            position: position.into(),
        }
    }

    /// File identifier of the sticker
    #[must_use]
    pub fn sticker<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.sticker = val.into();
        self
    }

    /// New sticker position in the set, zero-based
    #[must_use]
    pub fn position<T: Into<i64>>(mut self, val: T) -> Self {
        self.position = val.into();
        self
    }
}
impl super::TelegramMethod for SetStickerPositionInSet {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setStickerPositionInSet", self, None)
    }
}
