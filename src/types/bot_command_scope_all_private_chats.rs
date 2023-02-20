use serde::{Deserialize, Serialize};

/// Represents the [`scope`](https://core.telegram.org/bots/api#botcommandscope) of bot commands, covering all private chats.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscopeallprivatechats>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeAllPrivateChats {
    /// Scope type, must be *all_private_chats*
    #[serde(rename = "type", default = "all_private_chats")]
    pub scope_type: String,
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

fn all_private_chats() -> String {
    "all_private_chats".to_string()
}
