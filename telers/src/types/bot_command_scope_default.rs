use serde::{Deserialize, Serialize};
/// Represents the default scope of bot commands. Default commands are used if no commands with a narrower scope are specified for the user.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscopedefault>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BotCommandScopeDefault {}
impl BotCommandScopeDefault {
    /// Creates a new `BotCommandScopeDefault`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for BotCommandScopeDefault {
    fn default() -> Self {
        Self::new()
    }
}
