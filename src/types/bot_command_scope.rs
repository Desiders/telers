use serde::{Deserialize, Serialize};

use super::{
    BotCommandScopeAllChatAdministrators, BotCommandScopeAllGroupChats,
    BotCommandScopeAllPrivateChats, BotCommandScopeChat, BotCommandScopeChatAdministrators,
    BotCommandScopeChatMember, BotCommandScopeDefault,
};

/// This object represents the scope to which bot commands are applied. Currently, the following 7 scopes are supported:
/// - `aiogram_rs.types.bot_command_scope_default.BotCommandScopeDefault`
/// - `aiogram_rs.types.bot_command_scope_all_private_chats.BotCommandScopeAllPrivateChats`
/// - `aiogram_rs.types.bot_command_scope_all_group_chats.BotCommandScopeAllGroupChats`
/// - `aiogram_rs.types.bot_command_scope_all_chat_administrators.BotCommandScopeAllChatAdministrators`
/// - `aiogram_rs.types.bot_command_scope_chat.BotCommandScopeChat`
/// - `aiogram_rs.types.bot_command_scope_chat_administrators.BotCommandScopeChatAdministrators`
/// - `aiogram_rs.types.bot_command_scope_chat_member.BotCommandScopeChatMember`
/// <https://core.telegram.org/bots/api#botcommandscope>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
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
