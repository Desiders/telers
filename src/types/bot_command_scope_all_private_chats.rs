use serde::{Deserialize, Serialize};

/// Represents the [`scope`](https://core.telegram.org/bots/api#botcommandscope) of bot commands, covering all private chats.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscopeallprivatechats>
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct BotCommandScopeAllPrivateChats;

impl BotCommandScopeAllPrivateChats {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}
