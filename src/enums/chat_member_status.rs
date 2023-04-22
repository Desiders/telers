use std::fmt::{self, Debug};

/// This enum represents all possible types of the chat member status
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmember>
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ChatMemberStatus {
    Creator,
    Administrator,
    Member,
    Restricted,
    Left,
    Kicked,
}

impl Debug for ChatMemberStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ChatMemberStatus {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            ChatMemberStatus::Creator => "creator",
            ChatMemberStatus::Administrator => "administrator",
            ChatMemberStatus::Member => "member",
            ChatMemberStatus::Restricted => "restricted",
            ChatMemberStatus::Left => "left",
            ChatMemberStatus::Kicked => "kicked",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [ChatMemberStatus; 6] {
        &[
            ChatMemberStatus::Creator,
            ChatMemberStatus::Administrator,
            ChatMemberStatus::Member,
            ChatMemberStatus::Restricted,
            ChatMemberStatus::Left,
            ChatMemberStatus::Kicked,
        ]
    }
}

impl<'a> PartialEq<&'a str> for ChatMemberStatus {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str() == *other
    }
}

impl From<ChatMemberStatus> for String {
    fn from(status: ChatMemberStatus) -> Self {
        status.as_str().to_string()
    }
}
