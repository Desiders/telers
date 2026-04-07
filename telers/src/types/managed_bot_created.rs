use serde::{Deserialize, Serialize};
/// This object contains information about the bot that was created to be managed by the current bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#managedbotcreated>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ManagedBotCreated {
    /// Information about the bot. The bot's token can be fetched using the method getManagedBotToken.
    pub bot: Box<crate::types::User>,
}
impl ManagedBotCreated {
    /// Creates a new `ManagedBotCreated`.
    ///
    /// # Arguments
    /// * `bot` - Information about the bot. The bot's token can be fetched using the method getManagedBotToken.
    #[must_use]
    pub fn new<T0: Into<crate::types::User>>(bot: T0) -> Self {
        Self {
            bot: Box::new(bot.into()),
        }
    }

    /// Information about the bot. The bot's token can be fetched using the method getManagedBotToken.
    #[must_use]
    pub fn bot<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.bot = Box::new(val.into());
        this
    }
}
