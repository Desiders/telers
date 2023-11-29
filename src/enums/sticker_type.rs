use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// This enum represents all possible types of the sticker
/// # Documentation
/// <https://core.telegram.org/bots/api#sticker>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum StickerType {
    #[strum(serialize = "regular")]
    Regular,
    #[strum(serialize = "mask")]
    Mask,
    #[strum(serialize = "custom_emoji")]
    CustomEmoji,
}

impl StickerType {
    #[must_use]
    pub const fn all() -> [StickerType; 3] {
        [
            StickerType::Regular,
            StickerType::Mask,
            StickerType::CustomEmoji,
        ]
    }
}

impl From<StickerType> for Box<str> {
    fn from(action: StickerType) -> Self {
        Into::<&'static str>::into(action).into()
    }
}

impl<'a> PartialEq<&'a str> for StickerType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
