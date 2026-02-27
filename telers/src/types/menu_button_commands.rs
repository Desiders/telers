use serde::{Deserialize, Serialize};
/// Represents a menu button, which opens the bot's list of commands.
/// # Documentation
/// <https://core.telegram.org/bots/api#menubuttoncommands>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MenuButtonCommands {}
impl MenuButtonCommands {
    /// Creates a new `MenuButtonCommands`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for MenuButtonCommands {
    fn default() -> Self {
        Self::new()
    }
}
