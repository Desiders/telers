use crate::client::Bot;
use serde::Serialize;
/// Use this method to set the thumbnail of a regular or mask sticker set. The format of the thumbnail file must match the format of the stickers in the set. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setstickersetthumbnail>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetStickerSetThumbnail {
    /// Sticker set name
    pub name: Box<str>,
    /// User identifier of the sticker set owner
    pub user_id: i64,
    /// A .WEBP or .PNG image with the thumbnail, must be up to 128 kilobytes in size and have a width and height of exactly 100px, or a .TGS animation with a thumbnail up to 32 kilobytes in size (see <https://core.telegram.org/stickers#animation-requirements> for animated sticker technical requirements), or a .WEBM video with the thumbnail up to 32 kilobytes in size; see <https://core.telegram.org/stickers#video-requirements> for video sticker technical requirements. Pass a `file_id` as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>. Animated and video sticker set thumbnails can't be uploaded via HTTP URL. If omitted, then the thumbnail is dropped and the first sticker is used as the thumbnail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<crate::types::InputFile>,
    /// Format of the thumbnail, must be one of `static` for a .WEBP or .PNG image, `animated` for a .TGS animation, or `video` for a .WEBM video
    pub format: Box<str>,
}
impl SetStickerSetThumbnail {
    /// Creates a new `SetStickerSetThumbnail`.
    ///
    /// # Arguments
    /// * `name` - Sticker set name
    /// * `user_id` - User identifier of the sticker set owner
    /// * `format` - Format of the thumbnail, must be one of `static` for a .WEBP or .PNG image, `animated` for a .TGS animation, or `video` for a .WEBM video
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<i64>, T2: Into<Box<str>>>(
        name: T0,
        user_id: T1,
        format: T2,
    ) -> Self {
        Self {
            name: name.into(),
            user_id: user_id.into(),
            thumbnail: None,
            format: format.into(),
        }
    }

    /// Sticker set name
    #[must_use]
    pub fn name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.name = val.into();
        this
    }

    /// User identifier of the sticker set owner
    #[must_use]
    pub fn user_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.user_id = val.into();
        this
    }

    /// A .WEBP or .PNG image with the thumbnail, must be up to 128 kilobytes in size and have a width and height of exactly 100px, or a .TGS animation with a thumbnail up to 32 kilobytes in size (see <https://core.telegram.org/stickers#animation-requirements> for animated sticker technical requirements), or a .WEBM video with the thumbnail up to 32 kilobytes in size; see <https://core.telegram.org/stickers#video-requirements> for video sticker technical requirements. Pass a `file_id` as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>. Animated and video sticker set thumbnails can't be uploaded via HTTP URL. If omitted, then the thumbnail is dropped and the first sticker is used as the thumbnail.
    #[must_use]
    pub fn thumbnail<T: Into<crate::types::InputFile>>(self, val: T) -> Self {
        let mut this = self;
        this.thumbnail = Some(val.into());
        this
    }

    /// A .WEBP or .PNG image with the thumbnail, must be up to 128 kilobytes in size and have a width and height of exactly 100px, or a .TGS animation with a thumbnail up to 32 kilobytes in size (see <https://core.telegram.org/stickers#animation-requirements> for animated sticker technical requirements), or a .WEBM video with the thumbnail up to 32 kilobytes in size; see <https://core.telegram.org/stickers#video-requirements> for video sticker technical requirements. Pass a `file_id` as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>. Animated and video sticker set thumbnails can't be uploaded via HTTP URL. If omitted, then the thumbnail is dropped and the first sticker is used as the thumbnail.
    #[must_use]
    pub fn thumbnail_option<T: Into<crate::types::InputFile>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.thumbnail = val.map(Into::into);
        this
    }

    /// Format of the thumbnail, must be one of `static` for a .WEBP or .PNG image, `animated` for a .TGS animation, or `video` for a .WEBM video
    #[must_use]
    pub fn format<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.format = val.into();
        this
    }
}
impl super::TelegramMethod for SetStickerSetThumbnail {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(mut self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        let mut files = vec![];
        if let Some(file) = &mut self.thumbnail {
            super::prepare_file(&mut files, file);
        }
        super::Request::new("setStickerSetThumbnail", self, Some(files))
    }
}
