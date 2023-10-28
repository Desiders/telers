use super::{
    ChatMemberAdministrator, ChatMemberBanned, ChatMemberLeft, ChatMemberMember, ChatMemberOwner,
    ChatMemberRestricted,
};

use serde::Deserialize;

/// This object contains information about one member of a chat. Currently, the following 6 types of chat members are supported:
/// - [`ChatMemberOwner`]
/// - [`ChatMemberAdministrator`]
/// - [`ChatMemberMember`]
/// - [`ChatMemberRestricted`]
/// - [`ChatMemberLeft`]
/// - [`ChatMemberBanned`]
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmember>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "status")]
pub enum ChatMember {
    #[serde(rename = "creator")]
    Owner(ChatMemberOwner),
    Administrator(ChatMemberAdministrator),
    Member(ChatMemberMember),
    Restricted(ChatMemberRestricted),
    Left(ChatMemberLeft),
    #[serde(rename = "kicked")]
    Banned(ChatMemberBanned),
}

impl From<ChatMemberOwner> for ChatMember {
    fn from(chat_member: ChatMemberOwner) -> Self {
        Self::Owner(chat_member)
    }
}

impl From<ChatMemberAdministrator> for ChatMember {
    fn from(chat_member: ChatMemberAdministrator) -> Self {
        Self::Administrator(chat_member)
    }
}

impl From<ChatMemberMember> for ChatMember {
    fn from(chat_member: ChatMemberMember) -> Self {
        Self::Member(chat_member)
    }
}

impl From<ChatMemberRestricted> for ChatMember {
    fn from(chat_member: ChatMemberRestricted) -> Self {
        Self::Restricted(chat_member)
    }
}

impl From<ChatMemberLeft> for ChatMember {
    fn from(chat_member: ChatMemberLeft) -> Self {
        Self::Left(chat_member)
    }
}

impl From<ChatMemberBanned> for ChatMember {
    fn from(chat_member: ChatMemberBanned) -> Self {
        Self::Banned(chat_member)
    }
}
