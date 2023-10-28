use serde::{Deserialize, Serialize};

/// This object represents a bot command.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommand>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct BotCommand {
    /// Text of the command, 1-32 characters. Can contain only lowercase English letters, digits and underscores.
    pub command: String,
    /// Description of the command, 3-256 characters.
    pub description: String,
}

impl BotCommand {
    #[must_use]
    pub fn new(command: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            command: command.into(),
            description: description.into(),
        }
    }

    #[must_use]
    pub fn command(self, val: impl Into<String>) -> Self {
        Self {
            command: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn description(self, val: impl Into<String>) -> Self {
        Self {
            description: val.into(),
            ..self
        }
    }
}

impl<T: Into<String>> From<[T; 2]> for BotCommand {
    fn from([command, description]: [T; 2]) -> Self {
        Self {
            command: command.into(),
            description: description.into(),
        }
    }
}

impl<T: Into<String>> From<(T, T)> for BotCommand {
    fn from((command, description): (T, T)) -> Self {
        Self {
            command: command.into(),
            description: description.into(),
        }
    }
}
