use super::{
    ChatMemberAdministrator, ChatMemberBanned, ChatMemberLeft, ChatMemberMember, ChatMemberOwner,
    ChatMemberRestricted, User,
};

use serde::Deserialize;
use strum_macros::Display;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct ChatMember {
    /// Information about the user.
    pub user: User,

    #[serde(flatten)]
    pub kind: Kind,
}

/// This object contains information about one member of a chat. Currently, the following 6 types of chat members are supported:
/// - [`ChatMemberOwner`]
/// - [`ChatMemberAdministrator`]
/// - [`ChatMemberMember`]
/// - [`ChatMemberRestricted`]
/// - [`ChatMemberLeft`]
/// - [`ChatMemberBanned`]
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmember>
#[derive(Debug, Display, Clone, Hash, PartialEq, Eq, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Kind {
    #[serde(rename = "creator")]
    Owner(ChatMemberOwner),
    Administrator(ChatMemberAdministrator),
    Member(ChatMemberMember),
    Restricted(ChatMemberRestricted),
    Left(ChatMemberLeft),
    #[serde(rename = "kicked")]
    Banned(ChatMemberBanned),
}

impl From<ChatMemberOwner> for Kind {
    fn from(chat_member: ChatMemberOwner) -> Self {
        Self::Owner(chat_member)
    }
}

impl From<ChatMemberAdministrator> for Kind {
    fn from(chat_member: ChatMemberAdministrator) -> Self {
        Self::Administrator(chat_member)
    }
}

impl From<ChatMemberMember> for Kind {
    fn from(chat_member: ChatMemberMember) -> Self {
        Self::Member(chat_member)
    }
}

impl From<ChatMemberRestricted> for Kind {
    fn from(chat_member: ChatMemberRestricted) -> Self {
        Self::Restricted(chat_member)
    }
}

impl From<ChatMemberLeft> for Kind {
    fn from(chat_member: ChatMemberLeft) -> Self {
        Self::Left(chat_member)
    }
}

impl From<ChatMemberBanned> for Kind {
    fn from(chat_member: ChatMemberBanned) -> Self {
        Self::Banned(chat_member)
    }
}

impl ChatMember {
    #[must_use]
    pub const fn kind(&self) -> &Kind {
        &self.kind
    }
}
