use serde::{Deserialize, Serialize};

/// Represents a menu button, which opens the bot's list of commands.
/// <https://core.telegram.org/bots/api#menubuttoncommands>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct MenuButtonCommands {
    /// Type of the button, must be *commands*
    #[serde(rename = "type", default = "commands")]
    pub button_type: String,
}

fn commands() -> String {
    "commands".to_string()
}
