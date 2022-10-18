use super::{PhotoSize, Sticker};

use serde::{Deserialize, Serialize};

/// This object represents a sticker set.
/// <https://core.telegram.org/bots/api#stickerset>_
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StickerSet {
    /// Sticker set name
    pub name: String,
    /// Sticker set title
    pub title: String,
    /// Type of stickers in the set, currently one of 'regular', 'mask', 'custom_emoji'
    pub sticker_type: String,
    /// :code:`True`, if the sticker set contains `animated stickers <https://telegram.org/blog/animated-stickers>`_
    pub is_animated: bool,
    /// :code:`True`, if the sticker set contains `video stickers <https://telegram.org/blog/video-stickers-better-reactions>`_
    pub is_video: bool,
    /// List of all set stickers
    pub stickers: Vec<Sticker>,
    /// *Optional*. Sticker set thumbnail in the .WEBP, .TGS, or .WEBM format
    pub thumb: Option<PhotoSize>,
}
