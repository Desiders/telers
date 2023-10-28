use std::{
    fmt::{self, Debug, Display},
    ops::Deref,
};

/// This enum represents all possible types of the sticker
/// # Documentation
/// <https://core.telegram.org/bots/api#sticker>
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum StickerFormat {
    Static,
    Animated,
    Video,
}

impl StickerFormat {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            StickerFormat::Static => "static",
            StickerFormat::Animated => "animated",
            StickerFormat::Video => "video",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [StickerFormat; 3] {
        &[
            StickerFormat::Static,
            StickerFormat::Animated,
            StickerFormat::Video,
        ]
    }
}

impl Deref for StickerFormat {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Display for StickerFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<StickerFormat> for Box<str> {
    fn from(format: StickerFormat) -> Self {
        format.into()
    }
}

impl From<StickerFormat> for String {
    fn from(format: StickerFormat) -> Self {
        format.as_str().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for StickerFormat {
    fn eq(&self, other: &&'a str) -> bool {
        self == other
    }
}
