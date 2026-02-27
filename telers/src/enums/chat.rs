use crate::types::Chat;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object represents a chat.
/// Currently, it can be one of
/// - [`ChatChannel`]
/// - [`ChatGroup`]
/// - [`ChatPrivate`]
/// - [`ChatSupergroup`]
/// # Documentation
/// <https://core.telegram.org/bots/api#chat>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum ChatType {
    #[strum(serialize = "private")]
    Private,
    #[strum(serialize = "group")]
    Group,
    #[strum(serialize = "supergroup")]
    Supergroup,
    #[strum(serialize = "channel")]
    Channel,
}
impl ChatType {
    #[must_use]
    pub const fn all() -> [ChatType; 4usize] {
        [
            ChatType::Private,
            ChatType::Group,
            ChatType::Supergroup,
            ChatType::Channel,
        ]
    }
}
impl From<ChatType> for Box<str> {
    fn from(val: ChatType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<ChatType> for String {
    fn from(val: ChatType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for ChatType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a Chat> for ChatType {
    fn from(val: &'a Chat) -> Self {
        match val {
            Chat::Private(_) => ChatType::Private,
            Chat::Group(_) => ChatType::Group,
            Chat::Supergroup(_) => ChatType::Supergroup,
            Chat::Channel(_) => ChatType::Channel,
        }
    }
}
impl From<Chat> for ChatType {
    fn from(val: Chat) -> Self {
        ChatType::from(&val)
    }
}
