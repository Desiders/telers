use std::fmt::{self, Debug};

/// This enum represents all possible types of the chat
/// # Documentation
/// <https://core.telegram.org/bots/api#chat>
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ChatType {
    Private,
    Group,
    Supergroup,
    Channel,
}

impl Debug for ChatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ChatType {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            ChatType::Private => "private",
            ChatType::Group => "group",
            ChatType::Supergroup => "supergroup",
            ChatType::Channel => "channel",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [ChatType; 4] {
        &[
            ChatType::Private,
            ChatType::Group,
            ChatType::Supergroup,
            ChatType::Channel,
        ]
    }
}

impl From<ChatType> for String {
    fn from(chat_type: ChatType) -> Self {
        chat_type.as_str().to_string()
    }
}
