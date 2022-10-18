use super::ChatIdKind;

use serde::{Deserialize, Serialize};

/// Represents the `scope <https://core.telegram.org/bots/api#botcommandscope>`_ of bot commands, covering a specific chat.
/// <https://core.telegram.org/bots/api#botcommandscopechat>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeChat {
    /// Scope type, must be *chat*
    #[serde(rename = "type", default = "chat")]
    pub scope_type: String,
    /// Unique identifier for the target chat or username of the target supergroup (in the format :code:`@supergroupusername`)
    pub chat_id: ChatIdKind,
}

impl Default for BotCommandScopeChat {
    fn default() -> Self {
        Self {
            scope_type: chat(),
            chat_id: ChatIdKind::Id(0),
        }
    }
}

fn chat() -> String {
    "chat".to_string()
}
