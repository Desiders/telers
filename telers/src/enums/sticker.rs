use crate::types::Sticker;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object represents a sticker.
/// Currently, it can be one of
/// - [`StickerCustomEmoji`]
/// - [`StickerMask`]
/// - [`StickerRegular`]
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
    pub const fn all() -> [StickerType; 3usize] {
        [
            StickerType::Regular,
            StickerType::Mask,
            StickerType::CustomEmoji,
        ]
    }
}
impl From<StickerType> for Box<str> {
    fn from(val: StickerType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<StickerType> for String {
    fn from(val: StickerType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for StickerType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a Sticker> for StickerType {
    fn from(val: &'a Sticker) -> Self {
        match val {
            Sticker::Regular(_) => StickerType::Regular,
            Sticker::Mask(_) => StickerType::Mask,
            Sticker::CustomEmoji(_) => StickerType::CustomEmoji,
        }
    }
}
impl From<Sticker> for StickerType {
    fn from(val: Sticker) -> Self {
        StickerType::from(&val)
    }
}
