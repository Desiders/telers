use serde::{Deserialize, Serialize};
/// This object represents a mask sticker.
/// # Notes
/// This object represents a sticker from original sticker type `mask`.
/// # Documentation
/// <https://core.telegram.org/bots/api#sticker>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StickerMask {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: Box<str>,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: Box<str>,
    /// Sticker width
    pub width: i64,
    /// Sticker height
    pub height: i64,
    /// `true`, if the sticker is animated
    pub is_animated: bool,
    /// `true`, if the sticker is a video sticker
    pub is_video: bool,
    /// Sticker thumbnail in the .WEBP or .JPG format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<crate::types::PhotoSize>,
    /// Emoji associated with the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<Box<str>>,
    /// Name of the sticker set to which the sticker belongs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_name: Option<Box<str>>,
    /// For mask stickers, the position where the mask should be placed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<crate::types::MaskPosition>,
    /// For custom emoji stickers, unique identifier of the custom emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<Box<str>>,
    /// `true`, if the sticker must be repainted to a text color in messages, the color of the Telegram Premium badge in emoji status, white color on chat photos, or another appropriate color in other places
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<bool>,
    /// File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
impl StickerMask {
    /// Creates a new `StickerMask`.
    ///
    /// # Arguments
    /// * `file_id` - Identifier for this file, which can be used to download or reuse the file
    /// * `file_unique_id` - Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    /// * `width` - Sticker width
    /// * `height` - Sticker height
    /// * `is_animated` - `true`, if the sticker is animated
    /// * `is_video` - `true`, if the sticker is a video sticker
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<Box<str>>,
        T2: Into<i64>,
        T3: Into<i64>,
        T4: Into<bool>,
        T5: Into<bool>,
    >(
        file_id: T0,
        file_unique_id: T1,
        width: T2,
        height: T3,
        is_animated: T4,
        is_video: T5,
    ) -> Self {
        Self {
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            width: width.into(),
            height: height.into(),
            is_animated: is_animated.into(),
            is_video: is_video.into(),
            thumbnail: None,
            emoji: None,
            set_name: None,
            mask_position: None,
            custom_emoji_id: None,
            needs_repainting: None,
            file_size: None,
        }
    }

    /// Identifier for this file, which can be used to download or reuse the file
    #[must_use]
    pub fn file_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.file_id = val.into();
        this
    }

    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    #[must_use]
    pub fn file_unique_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.file_unique_id = val.into();
        this
    }

    /// Sticker width
    #[must_use]
    pub fn width<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.width = val.into();
        this
    }

    /// Sticker height
    #[must_use]
    pub fn height<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.height = val.into();
        this
    }

    /// `true`, if the sticker is animated
    #[must_use]
    pub fn is_animated<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_animated = val.into();
        this
    }

    /// `true`, if the sticker is a video sticker
    #[must_use]
    pub fn is_video<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_video = val.into();
        this
    }

    /// Sticker thumbnail in the .WEBP or .JPG format
    #[must_use]
    pub fn thumbnail<T: Into<crate::types::PhotoSize>>(self, val: T) -> Self {
        let mut this = self;
        this.thumbnail = Some(val.into());
        this
    }

    /// Sticker thumbnail in the .WEBP or .JPG format
    #[must_use]
    pub fn thumbnail_option<T: Into<crate::types::PhotoSize>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.thumbnail = val.map(Into::into);
        this
    }

    /// Emoji associated with the sticker
    #[must_use]
    pub fn emoji<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.emoji = Some(val.into());
        this
    }

    /// Emoji associated with the sticker
    #[must_use]
    pub fn emoji_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.emoji = val.map(Into::into);
        this
    }

    /// Name of the sticker set to which the sticker belongs
    #[must_use]
    pub fn set_name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.set_name = Some(val.into());
        this
    }

    /// Name of the sticker set to which the sticker belongs
    #[must_use]
    pub fn set_name_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.set_name = val.map(Into::into);
        this
    }

    /// For mask stickers, the position where the mask should be placed
    #[must_use]
    pub fn mask_position<T: Into<crate::types::MaskPosition>>(self, val: T) -> Self {
        let mut this = self;
        this.mask_position = Some(val.into());
        this
    }

    /// For mask stickers, the position where the mask should be placed
    #[must_use]
    pub fn mask_position_option<T: Into<crate::types::MaskPosition>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.mask_position = val.map(Into::into);
        this
    }

    /// For custom emoji stickers, unique identifier of the custom emoji
    #[must_use]
    pub fn custom_emoji_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.custom_emoji_id = Some(val.into());
        this
    }

    /// For custom emoji stickers, unique identifier of the custom emoji
    #[must_use]
    pub fn custom_emoji_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.custom_emoji_id = val.map(Into::into);
        this
    }

    /// `true`, if the sticker must be repainted to a text color in messages, the color of the Telegram Premium badge in emoji status, white color on chat photos, or another appropriate color in other places
    #[must_use]
    pub fn needs_repainting<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.needs_repainting = Some(val.into());
        this
    }

    /// `true`, if the sticker must be repainted to a text color in messages, the color of the Telegram Premium badge in emoji status, white color on chat photos, or another appropriate color in other places
    #[must_use]
    pub fn needs_repainting_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.needs_repainting = val.map(Into::into);
        this
    }

    /// File size in bytes
    #[must_use]
    pub fn file_size<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.file_size = Some(val.into());
        this
    }

    /// File size in bytes
    #[must_use]
    pub fn file_size_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.file_size = val.map(Into::into);
        this
    }
}
