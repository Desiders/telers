use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

use crate::types::ChatMember;

/// This enum represents all possible types of the chat member status
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmember>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum ChatMemberStatus {
    /// This variant is called `creator` in the API, but using for [`ChatMember::Owner`]
    #[strum(serialize = "creator")]
    Creator,
    #[strum(serialize = "administrator")]
    Administrator,
    #[strum(serialize = "member")]
    Member,
    #[strum(serialize = "restricted")]
    Restricted,
    #[strum(serialize = "left")]
    Left,
    /// This variant is called `kicked` in the API, but using for [`ChatMember::Banned`]
    #[strum(serialize = "kicked")]
    Kicked,
}

impl ChatMemberStatus {
    #[must_use]
    pub const fn all() -> [ChatMemberStatus; 6] {
        [
            ChatMemberStatus::Creator,
            ChatMemberStatus::Administrator,
            ChatMemberStatus::Member,
            ChatMemberStatus::Restricted,
            ChatMemberStatus::Left,
            ChatMemberStatus::Kicked,
        ]
    }
}

impl From<ChatMemberStatus> for Box<str> {
    fn from(status: ChatMemberStatus) -> Self {
        Into::<&'static str>::into(status).into()
    }
}

impl From<ChatMemberStatus> for String {
    fn from(status: ChatMemberStatus) -> Self {
        status.as_ref().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for ChatMemberStatus {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}

impl<'a> From<&'a ChatMember> for ChatMemberStatus {
    fn from(chat_member: &'a ChatMember) -> Self {
        match chat_member {
            ChatMember::Owner(_) => Self::Creator,
            ChatMember::Administrator(_) => Self::Administrator,
            ChatMember::Member(_) => Self::Member,
            ChatMember::Restricted(_) => Self::Restricted,
            ChatMember::Left(_) => Self::Left,
            ChatMember::Banned(_) => Self::Kicked,
        }
    }
}
