use serde::{Deserialize, Serialize};

/// Represents the [`scope`](https://core.telegram.org/bots/api#botcommandscope) of bot commands, covering all group and supergroup chat administrators.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscopeallchatadministrators>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct BotCommandScopeAllChatAdministrators;

impl BotCommandScopeAllChatAdministrators {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}
