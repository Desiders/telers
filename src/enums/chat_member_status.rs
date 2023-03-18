use std::fmt::{self, Debug};

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

impl From<ChatMemberStatus> for String {
    fn from(status: ChatMemberStatus) -> Self {
        status.as_str().to_string()
    }
}

impl<'a> From<&'a ChatMemberStatus> for String {
    fn from(status: &'a ChatMemberStatus) -> Self {
        status.as_str().to_string()
    }
}