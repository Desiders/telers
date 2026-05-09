use crate::client::Bot;
use serde::Serialize;
/// Use this method to set the thumbnail of a custom emoji sticker set. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setcustomemojistickersetthumbnail>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetCustomEmojiStickerSetThumbnail {
    /// Sticker set name
    pub name: Box<str>,
    /// Custom emoji identifier of a sticker from the sticker set; pass an empty string to drop the thumbnail and use the first sticker as the thumbnail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<Box<str>>,
}
impl SetCustomEmojiStickerSetThumbnail {
    /// Creates a new `SetCustomEmojiStickerSetThumbnail`.
    ///
    /// # Arguments
    /// * `name` - Sticker set name
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(name: T0) -> Self {
        Self {
            name: name.into(),
            custom_emoji_id: None,
        }
    }

    /// Sticker set name
    #[must_use]
    pub fn name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.name = val.into();
        self
    }

    /// Custom emoji identifier of a sticker from the sticker set; pass an empty string to drop the thumbnail and use the first sticker as the thumbnail.
    #[must_use]
    pub fn custom_emoji_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.custom_emoji_id = Some(val.into());
        self
    }

    /// Custom emoji identifier of a sticker from the sticker set; pass an empty string to drop the thumbnail and use the first sticker as the thumbnail.
    #[must_use]
    pub fn custom_emoji_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.custom_emoji_id = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for SetCustomEmojiStickerSetThumbnail {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setCustomEmojiStickerSetThumbnail", self, None)
    }
}
