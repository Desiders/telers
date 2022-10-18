use super::ChatIdKind;

use serde::{Deserialize, Serialize};

/// Represents the `scope <https://core.telegram.org/bots/api#botcommandscope>`_ of bot commands, covering all administrators of a specific group or supergroup chat.
/// <https://core.telegram.org/bots/api#botcommandscopechatadministrators>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeChatAdministrators {
    /// Scope type, must be *chat_administrators*
    #[serde(rename = "type", default = "chat_administrators")]
    pub scope_type: String,
    /// Unique identifier for the target chat or username of the target supergroup (in the format :code:`@supergroupusername`)
    pub chat_id: ChatIdKind,
}

fn chat_administrators() -> String {
    "chat_administrators".to_string()
}
