use super::ChatIdKind;

use crate::enums::BotCommandScopeType;

use serde::{Deserialize, Serialize};

/// Represents the [`scope`](https://core.telegram.org/bots/api#botcommandscope) of bot commands, covering a specific chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscopechat>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeChat {
    /// Scope type, must be *chat*
    #[serde(rename = "type", default = "chat")]
    pub scope_type: String,
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatIdKind,
}

impl BotCommandScopeChat {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>) -> Self {
        Self {
            scope_type: chat(),
            chat_id: chat_id.into(),
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: val.into(),
            ..self
        }
    }
}

fn chat() -> String {
    BotCommandScopeType::Chat.into()
}
