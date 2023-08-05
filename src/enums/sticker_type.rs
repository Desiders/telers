use std::fmt::{self, Debug, Display};

/// This enum represents all possible types of the sticker
/// # Documentation
/// <https://core.telegram.org/bots/api#sticker>
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum StickerType {
    Regular,
    Mask,
    CustomEmoji,
}

impl StickerType {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            StickerType::Regular => "regular",
            StickerType::Mask => "mask",
            StickerType::CustomEmoji => "custom_emoji",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [StickerType; 3] {
        &[
            StickerType::Regular,
            StickerType::Mask,
            StickerType::CustomEmoji,
        ]
    }
}

impl Display for StickerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<StickerType> for String {
    fn from(action: StickerType) -> Self {
        action.as_str().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for StickerType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str() == *other
    }
}
