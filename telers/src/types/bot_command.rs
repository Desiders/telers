use serde::{Deserialize, Serialize};
/// This object represents a bot command.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommand>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BotCommand {
    /// Text of the command; 1-32 characters. Can contain only lowercase English letters, digits and underscores.
    pub command: Box<str>,
    /// Description of the command; 1-256 characters
    pub description: Box<str>,
}
impl BotCommand {
    /// Creates a new `BotCommand`.
    ///
    /// # Arguments
    /// * `command` - Text of the command; 1-32 characters. Can contain only lowercase English letters, digits and underscores.
    /// * `description` - Description of the command; 1-256 characters
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(command: T0, description: T1) -> Self {
        Self {
            command: command.into(),
            description: description.into(),
        }
    }

    /// Text of the command; 1-32 characters. Can contain only lowercase English letters, digits and underscores.
    #[must_use]
    pub fn command<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.command = val.into();
        self
    }

    /// Description of the command; 1-256 characters
    #[must_use]
    pub fn description<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.description = val.into();
        self
    }
}
