use serde::{Deserialize, Serialize};
/// Represents a sticker file to be sent.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputmediasticker>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputMediaSticker {
    /// File to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a .WEBP sticker from the Internet, or pass `attach://<file_attach_name>` to upload a new .WEBP, .TGS, or .WEBM sticker using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    pub media: crate::types::InputFile,
    /// Emoji associated with the sticker; only for just uploaded stickers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<Box<str>>,
}
impl InputMediaSticker {
    /// Creates a new `InputMediaSticker`.
    ///
    /// # Arguments
    /// * `media` - File to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a .WEBP sticker from the Internet, or pass `attach://<file_attach_name>` to upload a new .WEBP, .TGS, or .WEBM sticker using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::InputFile>>(media: T0) -> Self {
        Self {
            media: media.into(),
            emoji: None,
        }
    }

    /// File to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a .WEBP sticker from the Internet, or pass `attach://<file_attach_name>` to upload a new .WEBP, .TGS, or .WEBM sticker using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn media<T: Into<crate::types::InputFile>>(mut self, val: T) -> Self {
        self.media = val.into();
        self
    }

    /// Emoji associated with the sticker; only for just uploaded stickers
    #[must_use]
    pub fn emoji<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.emoji = Some(val.into());
        self
    }

    /// Emoji associated with the sticker; only for just uploaded stickers
    #[must_use]
    pub fn emoji_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.emoji = val.map(Into::into);
        self
    }
}
