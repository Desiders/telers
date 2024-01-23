use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// This enum represents all possible types of the sticker
/// # Documentation
/// <https://core.telegram.org/bots/api#sticker>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum StickerFormat {
    #[strum(serialize = "static")]
    Static,
    #[strum(serialize = "animated")]
    Animated,
    #[strum(serialize = "video")]
    Video,
}

impl StickerFormat {
    #[must_use]
    pub const fn all() -> [StickerFormat; 3] {
        [
            StickerFormat::Static,
            StickerFormat::Animated,
            StickerFormat::Video,
        ]
    }
}

impl From<StickerFormat> for Box<str> {
    fn from(format: StickerFormat) -> Self {
        Into::<&'static str>::into(format).into()
    }
}

impl From<StickerFormat> for String {
    fn from(format: StickerFormat) -> Self {
        format.as_ref().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for StickerFormat {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
