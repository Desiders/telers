use serde::{Deserialize, Serialize};
/// Represents the scope of bot commands, covering all private chats.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscopeallprivatechats>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BotCommandScopeAllPrivateChats {}
impl BotCommandScopeAllPrivateChats {
    /// Creates a new `BotCommandScopeAllPrivateChats`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for BotCommandScopeAllPrivateChats {
    fn default() -> Self {
        Self::new()
    }
}
