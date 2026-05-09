use crate::client::Bot;
use serde::Serialize;
/// Use this method to delete a sticker set that was created by the bot. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#deletestickerset>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct DeleteStickerSet {
    /// Sticker set name
    pub name: Box<str>,
}
impl DeleteStickerSet {
    /// Creates a new `DeleteStickerSet`.
    ///
    /// # Arguments
    /// * `name` - Sticker set name
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(name: T0) -> Self {
        Self {
            name: name.into(),
        }
    }

    /// Sticker set name
    #[must_use]
    pub fn name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.name = val.into();
        self
    }
}
impl super::TelegramMethod for DeleteStickerSet {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("deleteStickerSet", self, None)
    }
}
