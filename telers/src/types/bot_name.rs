use serde::{Deserialize, Serialize};
/// This object represents the bot's name.
/// # Documentation
/// <https://core.telegram.org/bots/api#botname>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BotName {
    /// The bot's name
    pub name: Box<str>,
}
impl BotName {
    /// Creates a new `BotName`.
    ///
    /// # Arguments
    /// * `name` - The bot's name
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(name: T0) -> Self {
        Self {
            name: name.into(),
        }
    }

    /// The bot's name
    #[must_use]
    pub fn name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.name = val.into();
        this
    }
}
