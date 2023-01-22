use super::{PhotoSize, Sticker};

use serde::Deserialize;

/// This object represents a sticker set.
/// # Documentation
/// <https://core.telegram.org/bots/api#stickerset>
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct StickerSet {
    /// Sticker set name
    pub name: String,
    /// Sticker set title
    pub title: String,
    /// Type of stickers in the set, currently one of 'regular', 'mask', 'custom_emoji'
    pub sticker_type: String,
    /// `True`, if the sticker set contains `animated stickers <https://telegram.org/blog/animated-stickers>`
    pub is_animated: bool,
    /// `True`, if the sticker set contains `video stickers <https://telegram.org/blog/video-stickers-better-reactions>`
    pub is_video: bool,
    /// List of all set stickers
    pub stickers: Vec<Sticker>,
    /// *Optional*. Sticker set thumbnail in the .WEBP, .TGS, or .WEBM format
    pub thumb: Option<PhotoSize>,
}
