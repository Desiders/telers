use serde::{Deserialize, Serialize};
/// This object represents a sticker.
/// Currently, it can be one of
/// - [`crate::types::StickerCustomEmoji`]
/// - [`crate::types::StickerMask`]
/// - [`crate::types::StickerRegular`]
/// # Documentation
/// <https://core.telegram.org/bots/api#sticker>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Sticker {
    Regular(crate::types::StickerRegular),
    Mask(crate::types::StickerMask),
    CustomEmoji(crate::types::StickerCustomEmoji),
    /// Content unknown to this version of the library
    #[serde(untagged)]
    Unknown(crate::types::StickerUnknown),
}
impl Sticker {
    /// Helper method for field `custom_emoji_id`.
    ///
    /// For custom emoji stickers, unique identifier of the custom emoji
    #[must_use]
    pub fn custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::CustomEmoji(val) => val.custom_emoji_id.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `emoji`.
    ///
    /// Emoji associated with the sticker
    #[must_use]
    pub fn emoji(&self) -> Option<&str> {
        match self {
            Self::Regular(val) => val.emoji.as_deref(),
            Self::Mask(val) => val.emoji.as_deref(),
            Self::CustomEmoji(val) => val.emoji.as_deref(),
            Self::Unknown(val) => val.emoji.as_deref(),
        }
    }

    /// Helper method for field `file_id`.
    ///
    /// Identifier for this file, which can be used to download or reuse the file
    #[must_use]
    pub fn file_id(&self) -> &str {
        match self {
            Self::Regular(val) => val.file_id.as_ref(),
            Self::Mask(val) => val.file_id.as_ref(),
            Self::CustomEmoji(val) => val.file_id.as_ref(),
            Self::Unknown(val) => val.file_id.as_ref(),
        }
    }

    /// Helper method for field `file_size`.
    ///
    /// File size in bytes
    #[must_use]
    pub fn file_size(&self) -> Option<i64> {
        match self {
            Self::Regular(val) => val.file_size,
            Self::Mask(val) => val.file_size,
            Self::CustomEmoji(val) => val.file_size,
            Self::Unknown(val) => val.file_size,
        }
    }

    /// Helper method for field `file_unique_id`.
    ///
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    #[must_use]
    pub fn file_unique_id(&self) -> &str {
        match self {
            Self::Regular(val) => val.file_unique_id.as_ref(),
            Self::Mask(val) => val.file_unique_id.as_ref(),
            Self::CustomEmoji(val) => val.file_unique_id.as_ref(),
            Self::Unknown(val) => val.file_unique_id.as_ref(),
        }
    }

    /// Helper method for field `height`.
    ///
    /// Sticker height
    #[must_use]
    pub fn height(&self) -> i64 {
        match self {
            Self::Regular(val) => val.height,
            Self::Mask(val) => val.height,
            Self::CustomEmoji(val) => val.height,
            Self::Unknown(val) => val.height,
        }
    }

    /// Helper method for field `is_animated`.
    ///
    /// `true`, if the sticker is animated
    #[must_use]
    pub fn is_animated(&self) -> bool {
        match self {
            Self::Regular(val) => val.is_animated,
            Self::Mask(val) => val.is_animated,
            Self::CustomEmoji(val) => val.is_animated,
            Self::Unknown(val) => val.is_animated,
        }
    }

    /// Helper method for field `is_video`.
    ///
    /// `true`, if the sticker is a video sticker
    #[must_use]
    pub fn is_video(&self) -> bool {
        match self {
            Self::Regular(val) => val.is_video,
            Self::Mask(val) => val.is_video,
            Self::CustomEmoji(val) => val.is_video,
            Self::Unknown(val) => val.is_video,
        }
    }

    /// Helper method for field `mask_position`.
    ///
    /// For mask stickers, the position where the mask should be placed
    #[must_use]
    pub fn mask_position(&self) -> Option<&crate::types::MaskPosition> {
        match self {
            Self::Mask(val) => val.mask_position.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `needs_repainting`.
    ///
    /// `true`, if the sticker must be repainted to a text color in messages, the color of the Telegram Premium badge in emoji status, white color on chat photos, or another appropriate color in other places
    #[must_use]
    pub fn needs_repainting(&self) -> Option<bool> {
        match self {
            Self::Regular(val) => val.needs_repainting,
            Self::Mask(val) => val.needs_repainting,
            Self::CustomEmoji(val) => val.needs_repainting,
            Self::Unknown(val) => val.needs_repainting,
        }
    }

    /// Helper method for field `premium_animation`.
    ///
    /// For premium regular stickers, premium animation for the sticker
    #[must_use]
    pub fn premium_animation(&self) -> Option<&crate::types::File> {
        match self {
            Self::Regular(val) => val.premium_animation.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `set_name`.
    ///
    /// Name of the sticker set to which the sticker belongs
    #[must_use]
    pub fn set_name(&self) -> Option<&str> {
        match self {
            Self::Regular(val) => val.set_name.as_deref(),
            Self::Mask(val) => val.set_name.as_deref(),
            Self::CustomEmoji(val) => val.set_name.as_deref(),
            Self::Unknown(val) => val.set_name.as_deref(),
        }
    }

    /// Helper method for field `thumbnail`.
    ///
    /// Sticker thumbnail in the .WEBP or .JPG format
    #[must_use]
    pub fn thumbnail(&self) -> Option<&crate::types::PhotoSize> {
        match self {
            Self::Regular(val) => val.thumbnail.as_ref(),
            Self::Mask(val) => val.thumbnail.as_ref(),
            Self::CustomEmoji(val) => val.thumbnail.as_ref(),
            Self::Unknown(val) => val.thumbnail.as_ref(),
        }
    }

    /// Helper method for field `width`.
    ///
    /// Sticker width
    #[must_use]
    pub fn width(&self) -> i64 {
        match self {
            Self::Regular(val) => val.width,
            Self::Mask(val) => val.width,
            Self::CustomEmoji(val) => val.width,
            Self::Unknown(val) => val.width,
        }
    }

    /// Helper method for nested field `file_path`.
    #[must_use]
    pub fn file_path(&self) -> Option<&str> {
        match self {
            Self::Regular(val) => val
                .premium_animation
                .as_ref()
                .and_then(|inner| inner.file_path.as_deref()),
            _ => None,
        }
    }

    /// Helper method for nested field `point`.
    #[must_use]
    pub fn point(&self) -> Option<&str> {
        match self {
            Self::Mask(val) => val.mask_position.as_ref().map(|inner| inner.point.as_ref()),
            _ => None,
        }
    }

    /// Helper method for nested field `scale`.
    #[must_use]
    pub fn scale(&self) -> Option<f64> {
        match self {
            Self::Mask(val) => val.mask_position.as_ref().map(|inner| inner.scale),
            _ => None,
        }
    }

    /// Helper method for nested field `x_shift`.
    #[must_use]
    pub fn x_shift(&self) -> Option<f64> {
        match self {
            Self::Mask(val) => val.mask_position.as_ref().map(|inner| inner.x_shift),
            _ => None,
        }
    }

    /// Helper method for nested field `y_shift`.
    #[must_use]
    pub fn y_shift(&self) -> Option<f64> {
        match self {
            Self::Mask(val) => val.mask_position.as_ref().map(|inner| inner.y_shift),
            _ => None,
        }
    }
}
impl crate::types::FileIdGetter for Sticker {
    fn file_id(&self) -> &str {
        Sticker::file_id(self)
    }
}
impl From<crate::types::StickerRegular> for Sticker {
    fn from(val: crate::types::StickerRegular) -> Self {
        Self::Regular(val)
    }
}
impl TryFrom<Sticker> for crate::types::StickerRegular {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Sticker) -> Result<Self, Self::Error> {
        if let Sticker::Regular(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Sticker),
                stringify!(StickerRegular),
            ))
        }
    }
}
impl From<crate::types::StickerMask> for Sticker {
    fn from(val: crate::types::StickerMask) -> Self {
        Self::Mask(val)
    }
}
impl TryFrom<Sticker> for crate::types::StickerMask {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Sticker) -> Result<Self, Self::Error> {
        if let Sticker::Mask(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Sticker),
                stringify!(StickerMask),
            ))
        }
    }
}
impl From<crate::types::StickerCustomEmoji> for Sticker {
    fn from(val: crate::types::StickerCustomEmoji) -> Self {
        Self::CustomEmoji(val)
    }
}
impl TryFrom<Sticker> for crate::types::StickerCustomEmoji {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Sticker) -> Result<Self, Self::Error> {
        if let Sticker::CustomEmoji(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Sticker),
                stringify!(StickerCustomEmoji),
            ))
        }
    }
}
impl From<crate::types::StickerUnknown> for Sticker {
    fn from(val: crate::types::StickerUnknown) -> Self {
        Self::Unknown(val)
    }
}
impl TryFrom<Sticker> for crate::types::StickerUnknown {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Sticker) -> Result<Self, Self::Error> {
        if let Sticker::Unknown(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Sticker),
                stringify!(StickerUnknown),
            ))
        }
    }
}
