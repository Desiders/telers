use serde::{Deserialize, Serialize};
/// Represents the scope of bot commands, covering all group and supergroup chat administrators.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscopeallchatadministrators>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BotCommandScopeAllChatAdministrators {}
impl BotCommandScopeAllChatAdministrators {
    /// Creates a new `BotCommandScopeAllChatAdministrators`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for BotCommandScopeAllChatAdministrators {
    fn default() -> Self {
        Self::new()
    }
}
