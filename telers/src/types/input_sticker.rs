use serde::{Deserialize, Serialize};
/// This object describes a sticker to be added to a sticker set.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputsticker>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputSticker {
    /// The added sticker. Pass a `file_id` as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new file using multipart/form-data under <`file_attach_name`> name. Animated and video stickers can't be uploaded via HTTP URL. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    pub sticker: crate::types::InputFile,
    /// Format of the added sticker, must be one of `static` for a .WEBP or .PNG image, `animated` for a .TGS animation, `video` for a .WEBM video
    pub format: Box<str>,
    /// List of 1-20 emoji associated with the sticker
    pub emoji_list: Box<[Box<str>]>,
    /// Position where the mask should be placed on faces. For `mask` stickers only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<crate::types::MaskPosition>,
    /// List of 0-20 search keywords for the sticker with total length of up to 64 characters. For `regular` and `custom_emoji` stickers only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Box<[Box<str>]>>,
}
impl InputSticker {
    /// Creates a new `InputSticker`.
    ///
    /// # Arguments
    /// * `sticker` - The added sticker. Pass a `file_id` as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new file using multipart/form-data under <`file_attach_name`> name. Animated and video stickers can't be uploaded via HTTP URL. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    /// * `format` - Format of the added sticker, must be one of `static` for a .WEBP or .PNG image, `animated` for a .TGS animation, `video` for a .WEBM video
    /// * `emoji_list` - List of 1-20 emoji associated with the sticker
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<crate::types::InputFile>,
        T1: Into<Box<str>>,
        T2Item: Into<Box<str>>,
        T2: IntoIterator<Item = T2Item>,
    >(
        sticker: T0,
        format: T1,
        emoji_list: T2,
    ) -> Self {
        Self {
            sticker: sticker.into(),
            format: format.into(),
            emoji_list: emoji_list.into_iter().map(Into::into).collect(),
            mask_position: None,
            keywords: None,
        }
    }

    /// The added sticker. Pass a `file_id` as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new file using multipart/form-data under <`file_attach_name`> name. Animated and video stickers can't be uploaded via HTTP URL. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn sticker<T: Into<crate::types::InputFile>>(self, val: T) -> Self {
        let mut this = self;
        this.sticker = val.into();
        this
    }

    /// Format of the added sticker, must be one of `static` for a .WEBP or .PNG image, `animated` for a .TGS animation, `video` for a .WEBM video
    #[must_use]
    pub fn format<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.format = val.into();
        this
    }

    /// List of 1-20 emoji associated with the sticker
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn emoji_lists<T: Into<Box<[Box<str>]>>>(self, val: T) -> Self {
        let mut this = self;
        this.emoji_list = this
            .emoji_list
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        this
    }

    /// List of 1-20 emoji associated with the sticker
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn emoji_list<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.emoji_list = this
            .emoji_list
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }

    /// Position where the mask should be placed on faces. For `mask` stickers only.
    #[must_use]
    pub fn mask_position<T: Into<crate::types::MaskPosition>>(self, val: T) -> Self {
        let mut this = self;
        this.mask_position = Some(val.into());
        this
    }

    /// Position where the mask should be placed on faces. For `mask` stickers only.
    #[must_use]
    pub fn mask_position_option<T: Into<crate::types::MaskPosition>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.mask_position = val.map(Into::into);
        this
    }

    /// List of 0-20 search keywords for the sticker with total length of up to 64 characters. For `regular` and `custom_emoji` stickers only.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn keywords<T: Into<Box<[Box<str>]>>>(self, val: T) -> Self {
        let mut this = self;
        this.keywords = Some(
            this.keywords
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        this
    }

    /// List of 0-20 search keywords for the sticker with total length of up to 64 characters. For `regular` and `custom_emoji` stickers only.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn keyword<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.keywords = Some(
            this.keywords
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// List of 0-20 search keywords for the sticker with total length of up to 64 characters. For `regular` and `custom_emoji` stickers only.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn keywords_option<T: Into<Box<[Box<str>]>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.keywords = val.map(Into::into);
        this
    }
}
