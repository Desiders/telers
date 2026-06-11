use serde::{Deserialize, Serialize};
/// A bot command.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtextbotcommand>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextBotCommand {
    /// The text
    pub text: Box<crate::types::RichText>,
    /// The bot command
    pub bot_command: Box<str>,
}
impl RichTextBotCommand {
    /// Creates a new `RichTextBotCommand`.
    ///
    /// # Arguments
    /// * `text` - The text
    /// * `bot_command` - The bot command
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>, T1: Into<Box<str>>>(
        text: T0,
        bot_command: T1,
    ) -> Self {
        Self {
            text: Box::new(text.into()),
            bot_command: bot_command.into(),
        }
    }

    /// The text
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }

    /// The bot command
    #[must_use]
    pub fn bot_command<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.bot_command = val.into();
        self
    }
}
