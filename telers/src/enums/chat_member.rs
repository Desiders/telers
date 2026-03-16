use crate::types::ChatMember;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object contains information about one member of a chat. Currently, the following 6 types of chat members are supported:
/// - [`crate::types::ChatMemberOwner`]
/// - [`crate::types::ChatMemberAdministrator`]
/// - [`crate::types::ChatMemberMember`]
/// - [`crate::types::ChatMemberRestricted`]
/// - [`crate::types::ChatMemberLeft`]
/// - [`crate::types::ChatMemberBanned`]
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmember>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum ChatMemberType {
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
    #[strum(serialize = "kicked")]
    Kicked,
}
impl ChatMemberType {
    #[must_use]
    pub const fn all() -> [ChatMemberType; 6usize] {
        [
            ChatMemberType::Creator,
            ChatMemberType::Administrator,
            ChatMemberType::Member,
            ChatMemberType::Restricted,
            ChatMemberType::Left,
            ChatMemberType::Kicked,
        ]
    }
}
impl From<ChatMemberType> for Box<str> {
    fn from(val: ChatMemberType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<ChatMemberType> for String {
    fn from(val: ChatMemberType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for ChatMemberType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a ChatMember> for ChatMemberType {
    fn from(val: &'a ChatMember) -> Self {
        match val {
            ChatMember::Creator(_) => ChatMemberType::Creator,
            ChatMember::Administrator(_) => ChatMemberType::Administrator,
            ChatMember::Member(_) => ChatMemberType::Member,
            ChatMember::Restricted(_) => ChatMemberType::Restricted,
            ChatMember::Left(_) => ChatMemberType::Left,
            ChatMember::Kicked(_) => ChatMemberType::Kicked,
        }
    }
}
impl From<ChatMember> for ChatMemberType {
    fn from(val: ChatMember) -> Self {
        ChatMemberType::from(&val)
    }
}
