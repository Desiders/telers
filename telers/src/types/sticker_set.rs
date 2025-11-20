use super::{PhotoSize, Sticker};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object represents a sticker set.
/// # Documentation
/// <https://core.telegram.org/bots/api#stickerset>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct StickerSet {
    /// Sticker set name
    pub name: Box<str>,
    /// Sticker set title
    pub title: Box<str>,
    /// Type of stickers in the set, currently one of `regular`, `mask`, `custom_emoji`
    pub sticker_type: Box<str>,
    /// List of all set stickers
    pub stickers: Box<[Sticker]>,
    /// Sticker set thumbnail in the .WEBP, .TGS, or .WEBM format
    pub thumbnail: Option<PhotoSize>,
}
