use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

use crate::types::Chat;

/// This enum represents all possible types of the chat
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
    pub const fn all() -> [ChatType; 4] {
        [
            ChatType::Private,
            ChatType::Group,
            ChatType::Supergroup,
            ChatType::Channel,
        ]
    }
}

impl From<ChatType> for Box<str> {
    fn from(chat_type: ChatType) -> Self {
        Into::<&'static str>::into(chat_type).into()
    }
}

impl<'a> PartialEq<&'a str> for ChatType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}

impl<'a> From<&'a Chat> for ChatType {
    fn from(chat: &'a Chat) -> Self {
        match chat {
            Chat::Private(_) => ChatType::Private,
            Chat::Group(_) => ChatType::Group,
            Chat::Supergroup(_) => ChatType::Supergroup,
            Chat::Channel(_) => ChatType::Channel,
        }
    }
}
