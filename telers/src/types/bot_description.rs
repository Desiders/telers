use serde::{Deserialize, Serialize};
/// This object represents the bot's description.
/// # Documentation
/// <https://core.telegram.org/bots/api#botdescription>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BotDescription {
    /// The bot's description
    pub description: Box<str>,
}
impl BotDescription {
    /// Creates a new `BotDescription`.
    ///
    /// # Arguments
    /// * `description` - The bot's description
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(description: T0) -> Self {
        Self {
            description: description.into(),
        }
    }

    /// The bot's description
    #[must_use]
    pub fn description<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.description = val.into();
        this
    }
}
