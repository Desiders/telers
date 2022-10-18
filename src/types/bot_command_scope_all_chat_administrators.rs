use serde::{Deserialize, Serialize};

/// Represents the `scope <https://core.telegram.org/bots/api#botcommandscope>`_ of bot commands, covering all group and supergroup chat administrators.
/// <https://core.telegram.org/bots/api#botcommandscopeallchatadministrators>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeAllChatAdministrators {
    /// Scope type, must be *all_chat_administrators*
    #[serde(rename = "type", default = "all_chat_administrators")]
    scope_type: String,
}

fn all_chat_administrators() -> String {
    "all_chat_administrators".to_string()
}
