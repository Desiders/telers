use crate::types::BotCommandScope;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object represents the scope to which bot commands are applied. Currently, the following 7 scopes are supported:
/// - [`crate::types::BotCommandScopeDefault`]
/// - [`crate::types::BotCommandScopeAllPrivateChats`]
/// - [`crate::types::BotCommandScopeAllGroupChats`]
/// - [`crate::types::BotCommandScopeAllChatAdministrators`]
/// - [`crate::types::BotCommandScopeChat`]
/// - [`crate::types::BotCommandScopeChatAdministrators`]
/// - [`crate::types::BotCommandScopeChatMember`]
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscope>
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
pub enum BotCommandScopeType {
    #[strum(serialize = "default")]
    Default,
    #[strum(serialize = "all_private_chats")]
    AllPrivateChats,
    #[strum(serialize = "all_group_chats")]
    AllGroupChats,
    #[strum(serialize = "all_chat_administrators")]
    AllChatAdministrators,
    #[strum(serialize = "chat")]
    Chat,
    #[strum(serialize = "chat_administrators")]
    ChatAdministrators,
    #[strum(serialize = "chat_member")]
    ChatMember,
}
impl BotCommandScopeType {
    #[must_use]
    pub const fn all() -> [BotCommandScopeType; 7usize] {
        [
            BotCommandScopeType::Default,
            BotCommandScopeType::AllPrivateChats,
            BotCommandScopeType::AllGroupChats,
            BotCommandScopeType::AllChatAdministrators,
            BotCommandScopeType::Chat,
            BotCommandScopeType::ChatAdministrators,
            BotCommandScopeType::ChatMember,
        ]
    }
}
impl From<BotCommandScopeType> for Box<str> {
    fn from(val: BotCommandScopeType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<BotCommandScopeType> for String {
    fn from(val: BotCommandScopeType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for BotCommandScopeType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a BotCommandScope> for BotCommandScopeType {
    fn from(val: &'a BotCommandScope) -> Self {
        match val {
            BotCommandScope::Default(_) => BotCommandScopeType::Default,
            BotCommandScope::AllPrivateChats(_) => BotCommandScopeType::AllPrivateChats,
            BotCommandScope::AllGroupChats(_) => BotCommandScopeType::AllGroupChats,
            BotCommandScope::AllChatAdministrators(_) => BotCommandScopeType::AllChatAdministrators,
            BotCommandScope::Chat(_) => BotCommandScopeType::Chat,
            BotCommandScope::ChatAdministrators(_) => BotCommandScopeType::ChatAdministrators,
            BotCommandScope::ChatMember(_) => BotCommandScopeType::ChatMember,
        }
    }
}
impl From<BotCommandScope> for BotCommandScopeType {
    fn from(val: BotCommandScope) -> Self {
        BotCommandScopeType::from(&val)
    }
}
