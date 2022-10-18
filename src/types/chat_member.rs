use super::{
    ChatMemberAdministrator, ChatMemberBanned, ChatMemberLeft, ChatMemberMember, ChatMemberOwner,
    ChatMemberRestricted,
};

use serde::{Deserialize, Serialize};

/// This object contains information about one member of a chat. Currently, the following 6 types of chat members are supported:
/// - :class:`aiogram_rs.types.chat_member_owner.ChatMemberOwner`
/// - :class:`aiogram_rs.types.chat_member_administrator.ChatMemberAdministrator`
/// - :class:`aiogram_rs.types.chat_member_member.ChatMemberMember`
/// - :class:`aiogram_rs.types.chat_member_restricted.ChatMemberRestricted`
/// - :class:`aiogram_rs.types.chat_member_left.ChatMemberLeft`
/// - :class:`aiogram_rs.types.chat_member_banned.ChatMemberBanned`
/// <https://core.telegram.org/bots/api#chatmember>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
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
