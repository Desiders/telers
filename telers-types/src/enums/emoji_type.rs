use crate::types::EmojiType;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object describes an emoji. Currently, it can be one of
/// - [`crate::types::EmojiTypeEmoji`]
/// - [`crate::types::EmojiTypeCustomEmoji`]
/// # Documentation
/// <https://core.telegram.org/bots/api#emojitype>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum EmojiTypeType {
    #[strum(serialize = "emoji")]
    Emoji,
    #[strum(serialize = "custom_emoji")]
    CustomEmoji,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl EmojiTypeType {
    #[must_use]
    pub const fn all() -> [EmojiTypeType; 3usize] {
        [
            EmojiTypeType::Emoji,
            EmojiTypeType::CustomEmoji,
            EmojiTypeType::Unknown,
        ]
    }
}
impl From<EmojiTypeType> for Box<str> {
    fn from(val: EmojiTypeType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<EmojiTypeType> for String {
    fn from(val: EmojiTypeType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for EmojiTypeType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a EmojiType> for EmojiTypeType {
    fn from(val: &'a EmojiType) -> Self {
        match val {
            EmojiType::Emoji(_) => EmojiTypeType::Emoji,
            EmojiType::CustomEmoji(_) => EmojiTypeType::CustomEmoji,
            EmojiType::Unknown(_) => EmojiTypeType::Unknown,
        }
    }
}
impl From<EmojiType> for EmojiTypeType {
    fn from(val: EmojiType) -> Self {
        EmojiTypeType::from(&val)
    }
}
