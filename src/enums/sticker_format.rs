use std::fmt::{self, Debug};

pub enum StickerFormat {
    Static,
    Animated,
    Video,
}

impl Debug for StickerFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl StickerFormat {
    pub const fn as_str(&self) -> &'static str {
        match self {
            StickerFormat::Static => "static",
            StickerFormat::Animated => "animated",
            StickerFormat::Video => "video",
        }
    }

    pub const fn all() -> &'static [StickerFormat; 3] {
        &[
            StickerFormat::Static,
            StickerFormat::Animated,
            StickerFormat::Video,
        ]
    }
}

impl From<StickerFormat> for String {
    fn from(format: StickerFormat) -> Self {
        format.as_str().to_string()
    }
}

impl<'a> From<&'a StickerFormat> for String {
    fn from(format: &'a StickerFormat) -> Self {
        format.as_str().to_string()
    }
}
