use crate::types::ChatFullInfo;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object contains full information about a chat.
/// Currently, it can be one of
/// - [`crate::types::ChatFullInfoChannel`]
/// - [`crate::types::ChatFullInfoGroup`]
/// - [`crate::types::ChatFullInfoPrivate`]
/// - [`crate::types::ChatFullInfoSupergroup`]
/// # Documentation
/// <https://core.telegram.org/bots/api#chatfullinfo>
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    AsRefStr,
    IntoStaticStr,
    Deserialize,
    Serialize,
)]
pub enum ChatFullInfoType {
    #[strum(serialize = "private")]
    Private,
    #[strum(serialize = "group")]
    Group,
    #[strum(serialize = "supergroup")]
    Supergroup,
    #[strum(serialize = "channel")]
    Channel,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl ChatFullInfoType {
    #[must_use]
    pub const fn all() -> [ChatFullInfoType; 5usize] {
        [
            ChatFullInfoType::Private,
            ChatFullInfoType::Group,
            ChatFullInfoType::Supergroup,
            ChatFullInfoType::Channel,
            ChatFullInfoType::Unknown,
        ]
    }
}
impl From<ChatFullInfoType> for Box<str> {
    fn from(val: ChatFullInfoType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<ChatFullInfoType> for String {
    fn from(val: ChatFullInfoType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for ChatFullInfoType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a ChatFullInfo> for ChatFullInfoType {
    fn from(val: &'a ChatFullInfo) -> Self {
        match val {
            ChatFullInfo::Private(_) => ChatFullInfoType::Private,
            ChatFullInfo::Group(_) => ChatFullInfoType::Group,
            ChatFullInfo::Supergroup(_) => ChatFullInfoType::Supergroup,
            ChatFullInfo::Channel(_) => ChatFullInfoType::Channel,
            ChatFullInfo::Unknown(_) => ChatFullInfoType::Unknown,
        }
    }
}
impl From<ChatFullInfo> for ChatFullInfoType {
    fn from(val: ChatFullInfo) -> Self {
        ChatFullInfoType::from(&val)
    }
}
