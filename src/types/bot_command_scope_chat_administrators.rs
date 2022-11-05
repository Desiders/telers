use super::ChatIdKind;

use serde::{Deserialize, Serialize};

/// Represents the `scope <https://core.telegram.org/bots/api#botcommandscope>` of bot commands, covering all administrators of a specific group or supergroup chat.
/// <https://core.telegram.org/bots/api#botcommandscopechatadministrators>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeChatAdministrators {
    /// Scope type, must be *chat_administrators*
    #[serde(rename = "type", default = "chat_administrators")]
    pub scope_type: String,
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatIdKind,
}

impl Default for BotCommandScopeChatAdministrators {
    fn default() -> Self {
        Self {
            scope_type: chat_administrators(),
            chat_id: ChatIdKind::Id(0),
        }
    }
}

fn chat_administrators() -> String {
    "chat_administrators".to_string()
}
