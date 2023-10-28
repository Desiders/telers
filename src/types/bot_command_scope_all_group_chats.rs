use serde::{Deserialize, Serialize};

use crate::enums::BotCommandScopeType;

/// Represents the [`scope`](https://core.telegram.org/bots/api#botcommandscope) of bot commands, covering all group and supergroup chats.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscopeallgroupchats>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeAllGroupChats {
    /// Scope type, must be *all_group_chats*
    #[serde(rename = "type", default = "all_group_chats")]
    pub scope_type: Box<str>,
}

impl BotCommandScopeAllGroupChats {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for BotCommandScopeAllGroupChats {
    #[must_use]
    fn default() -> Self {
        Self {
            scope_type: all_group_chats(),
        }
    }
}

fn all_group_chats() -> Box<str> {
    BotCommandScopeType::AllGroupChats.into()
}
