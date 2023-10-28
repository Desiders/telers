use serde::{Deserialize, Serialize};

use super::{
    BotCommandScopeAllChatAdministrators, BotCommandScopeAllGroupChats,
    BotCommandScopeAllPrivateChats, BotCommandScopeChat, BotCommandScopeChatAdministrators,
    BotCommandScopeChatMember, BotCommandScopeDefault,
};

/// This object represents the scope to which bot commands are applied. Currently, the following 7 scopes are supported:
/// - [`BotCommandScopeDefault`]
/// - [`BotCommandScopeAllPrivateChats`]
/// - [`BotCommandScopeAllGroupChats`]
/// - [`BotCommandScopeAllChatAdministrators`]
/// - [`BotCommandScopeChat`]
/// - [`BotCommandScopeChatAdministrators`]
/// - [`BotCommandScopeChatMember`]
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscope>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BotCommandScope {
    Default(BotCommandScopeDefault),
    AllPrivateChats(BotCommandScopeAllPrivateChats),
    AllGroupChats(BotCommandScopeAllGroupChats),
    AllChatAdministrators(BotCommandScopeAllChatAdministrators),
    Chat(BotCommandScopeChat),
    ChatAdministrators(BotCommandScopeChatAdministrators),
    ChatMember(BotCommandScopeChatMember),
}

impl From<BotCommandScopeDefault> for BotCommandScope {
    fn from(scope: BotCommandScopeDefault) -> Self {
        BotCommandScope::Default(scope)
    }
}

impl From<BotCommandScopeAllPrivateChats> for BotCommandScope {
    fn from(scope: BotCommandScopeAllPrivateChats) -> Self {
        BotCommandScope::AllPrivateChats(scope)
    }
}

impl From<BotCommandScopeAllGroupChats> for BotCommandScope {
    fn from(scope: BotCommandScopeAllGroupChats) -> Self {
        BotCommandScope::AllGroupChats(scope)
    }
}

impl From<BotCommandScopeAllChatAdministrators> for BotCommandScope {
    fn from(scope: BotCommandScopeAllChatAdministrators) -> Self {
        BotCommandScope::AllChatAdministrators(scope)
    }
}

impl From<BotCommandScopeChat> for BotCommandScope {
    fn from(scope: BotCommandScopeChat) -> Self {
        BotCommandScope::Chat(scope)
    }
}

impl From<BotCommandScopeChatAdministrators> for BotCommandScope {
    fn from(scope: BotCommandScopeChatAdministrators) -> Self {
        BotCommandScope::ChatAdministrators(scope)
    }
}

impl From<BotCommandScopeChatMember> for BotCommandScope {
    fn from(scope: BotCommandScopeChatMember) -> Self {
        BotCommandScope::ChatMember(scope)
    }
}
