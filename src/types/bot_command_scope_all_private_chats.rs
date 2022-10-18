use serde::{Deserialize, Serialize};

/// Represents the `scope <https://core.telegram.org/bots/api#botcommandscope>`_ of bot commands, covering all private chats.
/// <https://core.telegram.org/bots/api#botcommandscopeallprivatechats>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeAllPrivateChats {
    /// Scope type, must be *all_private_chats*
    #[serde(rename = "type", default = "all_private_chats")]
    scope_type: String,
}

fn all_private_chats() -> String {
    "all_private_chats".to_string()
}
