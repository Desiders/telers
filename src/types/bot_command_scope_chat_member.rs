use super::ChatIdKind;

use crate::enums::BotCommandScopeType;

use serde::{Deserialize, Serialize};

/// Represents the [`scope`](https://core.telegram.org/bots/api#botcommandscope) of bot commands, covering a specific member of a group or supergroup chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscopechatmember>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BotCommandScopeChatMember {
    /// Scope type, must be *chat_member*
    #[serde(rename = "type", default = "chat_member")]
    pub scope_type: Box<str>,
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
}

impl BotCommandScopeChatMember {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, user_id: i64) -> Self {
        Self {
            scope_type: chat_member(),
            chat_id: chat_id.into(),
            user_id,
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn user_id(self, val: i64) -> Self {
        Self {
            user_id: val,
            ..self
        }
    }
}

fn chat_member() -> Box<str> {
    BotCommandScopeType::ChatMember.into()
}
