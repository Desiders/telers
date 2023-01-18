use serde::{Deserialize, Serialize};

/// Represents the `scope <https://core.telegram.org/bots/api#botcommandscope>` of bot commands, covering all group and supergroup chat administrators.
/// <https://core.telegram.org/bots/api#botcommandscopeallchatadministrators>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeAllChatAdministrators {
    /// Scope type, must be *all_chat_administrators*
    #[serde(rename = "type", default = "all_chat_administrators")]
    pub scope_type: String,
}

impl BotCommandScopeAllChatAdministrators {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for BotCommandScopeAllChatAdministrators {
    fn default() -> Self {
        Self {
            scope_type: all_chat_administrators(),
        }
    }
}

fn all_chat_administrators() -> String {
    "all_chat_administrators".to_string()
}
