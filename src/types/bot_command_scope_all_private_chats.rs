use serde::{Deserialize, Serialize};

use crate::enums::BotCommandScopeType;

/// Represents the [`scope`](https://core.telegram.org/bots/api#botcommandscope) of bot commands, covering all private chats.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscopeallprivatechats>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct BotCommandScopeAllPrivateChats {
    /// Scope type, must be *all_private_chats*
    #[serde(rename = "type", default = "all_private_chats")]
    pub scope_type: Box<str>,
}

impl BotCommandScopeAllPrivateChats {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for BotCommandScopeAllPrivateChats {
    #[must_use]
    fn default() -> Self {
        Self {
            scope_type: all_private_chats(),
        }
    }
}

fn all_private_chats() -> Box<str> {
    BotCommandScopeType::AllPrivateChats.into()
}
