use super::ChatIdKind;

use crate::enums::BotCommandScopeType;

use serde::{Deserialize, Serialize};

/// Represents the [`scope`](https://core.telegram.org/bots/api#botcommandscope) of bot commands, covering all administrators of a specific group or supergroup chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscopechatadministrators>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeChatAdministrators {
    /// Scope type, must be *chat_administrators*
    #[serde(rename = "type", default = "chat_administrators")]
    pub scope_type: String,
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatIdKind,
}

impl BotCommandScopeChatAdministrators {
    #[must_use]
    pub fn new<T: Into<ChatIdKind>>(chat_id: T) -> Self {
        Self {
            scope_type: chat_administrators(),
            chat_id: chat_id.into(),
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }
}

fn chat_administrators() -> String {
    BotCommandScopeType::ChatAdministrators.into()
}
