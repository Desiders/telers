use serde::{Deserialize, Serialize};

/// This object represents a bot command.
/// <https://core.telegram.org/bots/api#botcommand>
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BotCommand {
    /// Text of the command, 1-32 characters. Can contain only lowercase English letters, digits and underscores.
    pub command: String,
    /// Description of the command, 3-256 characters.
    pub description: String,
}

impl BotCommand {
    #[must_use]
    pub fn new<T: Into<String>>(command: T, description: T) -> Self {
        Self {
            command: command.into(),
            description: description.into(),
        }
    }

    #[must_use]
    pub fn command<T: Into<String>>(mut self, val: T) -> Self {
        self.command = val.into();
        self
    }

    #[must_use]
    pub fn description<T: Into<String>>(mut self, val: T) -> Self {
        self.description = val.into();
        self
    }
}
