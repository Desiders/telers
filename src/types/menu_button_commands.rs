use serde::{Deserialize, Serialize};

/// Represents a menu button, which opens the bot's list of commands.
/// # Documentation
/// <https://core.telegram.org/bots/api#menubuttoncommands>
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct MenuButtonCommands;

impl MenuButtonCommands {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}
