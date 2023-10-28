use serde::{Deserialize, Serialize};

use crate::enums::BotCommandScopeType;

/// Represents the [`scope`](https://core.telegram.org/bots/api#botcommandscope) of bot commands, covering all group and supergroup chat administrators.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscopeallchatadministrators>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeAllChatAdministrators {
    /// Scope type, must be *all_chat_administrators*
    #[serde(rename = "type", default = "all_chat_administrators")]
    pub scope_type: Box<str>,
}

impl BotCommandScopeAllChatAdministrators {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for BotCommandScopeAllChatAdministrators {
    #[must_use]
    fn default() -> Self {
        Self {
            scope_type: all_chat_administrators(),
        }
    }
}

fn all_chat_administrators() -> Box<str> {
    BotCommandScopeType::AllChatAdministrators.into()
}
