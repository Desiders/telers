use crate::types::ChatShareTheme;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object describes the theme of the screen for sharing a link to a user or a chat. Currently, it can be one of
/// - [`crate::types::ChatShareThemeEmoji`]
/// - [`crate::types::ChatShareThemeUniqueGift`]
/// # Documentation
/// <https://core.telegram.org/bots/api#chatsharetheme>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
#[cfg_attr(feature = "deser", derive(deser::Deserialize, deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Deserialize, serde::Serialize))]
pub enum ChatShareThemeType {
    #[strum(serialize = "emoji")]
    Emoji,
    #[strum(serialize = "unique_gift")]
    UniqueGift,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl ChatShareThemeType {
    #[must_use]
    pub const fn all() -> [ChatShareThemeType; 3usize] {
        [
            ChatShareThemeType::Emoji,
            ChatShareThemeType::UniqueGift,
            ChatShareThemeType::Unknown,
        ]
    }
}
impl From<ChatShareThemeType> for Box<str> {
    fn from(val: ChatShareThemeType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<ChatShareThemeType> for String {
    fn from(val: ChatShareThemeType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for ChatShareThemeType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a ChatShareTheme> for ChatShareThemeType {
    fn from(val: &'a ChatShareTheme) -> Self {
        match val {
            ChatShareTheme::Emoji(_) => ChatShareThemeType::Emoji,
            ChatShareTheme::UniqueGift(_) => ChatShareThemeType::UniqueGift,
            ChatShareTheme::Unknown(_) => ChatShareThemeType::Unknown,
        }
    }
}
impl From<ChatShareTheme> for ChatShareThemeType {
    fn from(val: ChatShareTheme) -> Self {
        ChatShareThemeType::from(&val)
    }
}
