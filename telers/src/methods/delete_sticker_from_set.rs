use crate::client::Bot;
use serde::Serialize;
/// Use this method to delete a sticker from a set created by the bot. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#deletestickerfromset>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct DeleteStickerFromSet {
    /// File identifier of the sticker
    pub sticker: Box<str>,
}
impl DeleteStickerFromSet {
    /// Creates a new `DeleteStickerFromSet`.
    ///
    /// # Arguments
    /// * `sticker` - File identifier of the sticker
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(sticker: T0) -> Self {
        Self {
            sticker: sticker.into(),
        }
    }

    /// File identifier of the sticker
    #[must_use]
    pub fn sticker<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.sticker = val.into();
        self
    }
}
impl super::TelegramMethod for DeleteStickerFromSet {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("deleteStickerFromSet", self, None)
    }
}
