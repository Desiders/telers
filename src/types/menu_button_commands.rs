use crate::enums::MenuButtonType;

use serde::{Deserialize, Serialize};

/// Represents a menu button, which opens the bot's list of commands.
/// # Documentation
/// <https://core.telegram.org/bots/api#menubuttoncommands>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct MenuButtonCommands {
    /// Type of the button, must be *commands*
    #[serde(rename = "type", default = "commands")]
    pub button_type: Box<str>,
}

impl MenuButtonCommands {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for MenuButtonCommands {
    #[must_use]
    fn default() -> Self {
        Self {
            button_type: commands(),
        }
    }
}

fn commands() -> Box<str> {
    MenuButtonType::Commands.into()
}
