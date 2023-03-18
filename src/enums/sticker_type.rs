use std::fmt::{self, Debug};

pub enum StickerType {
    Regular,
    Mask,
    CustomEmoji,
}

impl Debug for StickerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl StickerType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            StickerType::Regular => "regular",
            StickerType::Mask => "mask",
            StickerType::CustomEmoji => "custom_emoji",
        }
    }

    pub const fn all() -> &'static [StickerType; 3] {
        &[
            StickerType::Regular,
            StickerType::Mask,
            StickerType::CustomEmoji,
        ]
    }
}

impl From<StickerType> for String {
    fn from(action: StickerType) -> Self {
        action.as_str().to_string()
    }
}

impl<'a> From<&'a StickerType> for String {
    fn from(action: &'a StickerType) -> Self {
        action.as_str().to_string()
    }
}