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
    /// `true`, if the command sends an ephemeral message, which can be seen only by the sender of the message and the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ephemeral: Option<bool>,
}
impl BotCommand {
    /// Creates a new `BotCommand`.
    ///
    /// # Arguments
    /// * `command` - Text of the command; 1-32 characters. Can contain only lowercase English letters, digits and underscores.
    /// * `description` - Description of the command; 1-256 characters
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(command: T0, description: T1) -> Self {
        Self {
            command: command.into(),
            description: description.into(),
            is_ephemeral: None,
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

    /// `true`, if the command sends an ephemeral message, which can be seen only by the sender of the message and the bot
    #[must_use]
    pub fn is_ephemeral<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_ephemeral = Some(val.into());
        self
    }

    /// `true`, if the command sends an ephemeral message, which can be seen only by the sender of the message and the bot
    #[must_use]
    pub fn is_ephemeral_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_ephemeral = val.map(Into::into);
        self
    }
}
