use super::{File, MaskPosition, PhotoSize};

use serde::{Deserialize, Serialize};

/// This object represents a sticker.
/// <https://core.telegram.org/bots/api#sticker>_
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sticker {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Type of the sticker, currently one of 'regular', 'mask', 'custom_emoji'. The type of the sticker is independent from its format, which is determined by the fields *is_animated* and *is_video*.
    #[serde(rename = "type")]
    pub sticker_type: String,
    /// Sticker width
    pub width: i64,
    /// Sticker height
    pub height: i64,
    /// :code:`True`, if the sticker is `animated <https://telegram.org/blog/animated-stickers>`_
    pub is_animated: bool,
    /// :code:`True`, if the sticker is a `video sticker <https://telegram.org/blog/video-stickers-better-reactions>`_
    pub is_video: bool,
    /// *Optional*. Sticker thumbnail in the .WEBP or .JPG format
    pub thumb: Option<PhotoSize>,
    /// *Optional*. Emoji associated with the sticker
    pub emoji: Option<String>,
    /// *Optional*. Name of the sticker set to which the sticker belongs
    pub set_name: Option<String>,
    /// *Optional*. For premium regular stickers, premium animation for the sticker
    pub premium_animation: Option<File>,
    /// *Optional*. For mask stickers, the position where the mask should be placed
    pub mask_position: Option<MaskPosition>,
    /// *Optional*. For custom emoji stickers, unique identifier of the custom emoji
    pub custom_emoji_id: Option<String>,
    /// *Optional*. File size in bytes
    pub file_size: Option<i64>,
}
