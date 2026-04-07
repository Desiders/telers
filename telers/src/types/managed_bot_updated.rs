use serde::{Deserialize, Serialize};
/// This object contains information about the creation or token update of a bot that is managed by the current bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#managedbotupdated>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ManagedBotUpdated {
    /// User that created the bot
    pub user: Box<crate::types::User>,
    /// Information about the bot. Token of the bot can be fetched using the method getManagedBotToken.
    pub bot: Box<crate::types::User>,
}
impl ManagedBotUpdated {
    /// Creates a new `ManagedBotUpdated`.
    ///
    /// # Arguments
    /// * `user` - User that created the bot
    /// * `bot` - Information about the bot. Token of the bot can be fetched using the method getManagedBotToken.
    #[must_use]
    pub fn new<T0: Into<crate::types::User>, T1: Into<crate::types::User>>(
        user: T0,
        bot: T1,
    ) -> Self {
        Self {
            user: Box::new(user.into()),
            bot: Box::new(bot.into()),
        }
    }

    /// User that created the bot
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.user = Box::new(val.into());
        this
    }

    /// Information about the bot. Token of the bot can be fetched using the method getManagedBotToken.
    #[must_use]
    pub fn bot<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.bot = Box::new(val.into());
        this
    }
}
