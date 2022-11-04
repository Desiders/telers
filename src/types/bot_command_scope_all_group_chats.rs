use serde::{Deserialize, Serialize};

/// Represents the `scope <https://core.telegram.org/bots/api#botcommandscope>` of bot commands, covering all group and supergroup chats.
/// <https://core.telegram.org/bots/api#botcommandscopeallgroupchats>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeAllGroupChats {
    /// Scope type, must be *all_group_chats*
    #[serde(rename = "type", default = "all_group_chats")]
    scope_type: String,
}

impl Default for BotCommandScopeAllGroupChats {
    fn default() -> Self {
        Self {
            scope_type: all_group_chats(),
        }
    }
}

fn all_group_chats() -> String {
    "all_group_chats".to_string()
}
