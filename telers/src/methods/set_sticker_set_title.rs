use crate::client::Bot;
use serde::Serialize;
/// Use this method to set the title of a created sticker set. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setstickersettitle>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetStickerSetTitle {
    /// Sticker set name
    pub name: Box<str>,
    /// Sticker set title, 1-64 characters
    pub title: Box<str>,
}
impl SetStickerSetTitle {
    /// Creates a new `SetStickerSetTitle`.
    ///
    /// # Arguments
    /// * `name` - Sticker set name
    /// * `title` - Sticker set title, 1-64 characters
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(name: T0, title: T1) -> Self {
        Self {
            name: name.into(),
            title: title.into(),
        }
    }

    /// Sticker set name
    #[must_use]
    pub fn name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.name = val.into();
        self
    }

    /// Sticker set title, 1-64 characters
    #[must_use]
    pub fn title<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.title = val.into();
        self
    }
}
impl super::TelegramMethod for SetStickerSetTitle {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setStickerSetTitle", self, None)
    }
}
