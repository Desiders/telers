use serde::{Deserialize, Serialize};
/// This object represents the bot's short description.
/// # Documentation
/// <https://core.telegram.org/bots/api#botshortdescription>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BotShortDescription {
    /// The bot's short description
    pub short_description: Box<str>,
}
impl BotShortDescription {
    /// Creates a new `BotShortDescription`.
    ///
    /// # Arguments
    /// * `short_description` - The bot's short description
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(short_description: T0) -> Self {
        Self {
            short_description: short_description.into(),
        }
    }

    /// The bot's short description
    #[must_use]
    pub fn short_description<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.short_description = val.into();
        this
    }
}
