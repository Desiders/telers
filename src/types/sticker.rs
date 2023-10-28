use super::{File, MaskPosition, PhotoSize};

use serde::Deserialize;

/// This object represents a sticker.
/// # Documentation
/// <https://core.telegram.org/bots/api#sticker>
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Sticker {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: Box<str>,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: Box<str>,
    /// Type of the sticker, currently one of 'regular', 'mask', 'custom_emoji'. The type of the sticker is independent from its format, which is determined by the fields *is_animated* and *is_video*.
    #[serde(rename = "type")]
    pub sticker_type: Box<str>,
    /// Sticker width
    pub width: i64,
    /// Sticker height
    pub height: i64,
    /// `True`, if the sticker is [`animated`](https://telegram.org/blog/animated-stickers)
    pub is_animated: bool,
    /// `True`, if the sticker is a [`video sticker`](https://telegram.org/blog/video-stickers-better-reactions)
    pub is_video: bool,
    /// Sticker thumbnail in the .WEBP or .JPG format
    pub thumbnail: Option<PhotoSize>,
    /// Emoji associated with the sticker
    pub emoji: Option<Box<str>>,
    /// Name of the sticker set to which the sticker belongs
    pub set_name: Option<Box<str>>,
    /// For premium regular stickers, premium animation for the sticker
    pub premium_animation: Option<File>,
    /// For mask stickers, the position where the mask should be placed
    pub mask_position: Option<MaskPosition>,
    /// For custom emoji stickers, unique identifier of the custom emoji
    pub custom_emoji_id: Option<Box<str>>,
    /// `True`, if the sticker must be repainted to a text color in messages, the color of the Telegram Premium badge in emoji status, white color on chat photos, or another appropriate color in other places
    pub needs_repainting: Option<bool>,
    /// File size in bytes
    pub file_size: Option<i64>,
}
