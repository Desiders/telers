use crate::client::Bot;
use serde::Serialize;
/// Use this method to get a sticker set. On success, a [`crate::types::StickerSet`] object is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#getstickerset>
/// # Returns
/// - `crate::types::StickerSet`
#[derive(Clone, Debug, Serialize)]
pub struct GetStickerSet {
    /// Name of the sticker set
    pub name: Box<str>,
}
impl GetStickerSet {
    /// Creates a new `GetStickerSet`.
    ///
    /// # Arguments
    /// * `name` - Name of the sticker set
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(name: T0) -> Self {
        Self {
            name: name.into(),
        }
    }

    /// Name of the sticker set
    #[must_use]
    pub fn name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.name = val.into();
        self
    }
}
impl super::TelegramMethod for GetStickerSet {
    type Method = Self;
    type Return = crate::types::StickerSet;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getStickerSet", self, None)
    }
}
