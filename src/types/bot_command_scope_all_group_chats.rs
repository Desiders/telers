use serde::{Deserialize, Serialize};

/// Represents the [`scope`](https://core.telegram.org/bots/api#botcommandscope) of bot commands, covering all group and supergroup chats.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscopeallgroupchats>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct BotCommandScopeAllGroupChats;

impl BotCommandScopeAllGroupChats {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}
