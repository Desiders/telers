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
/// <https://core.telegram.org/bots/api#botcommandscope>_
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
