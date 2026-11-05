/// This object contains information about the bot that was created to be managed by the current bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#managedbotcreated>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct ManagedBotCreated {
    /// Information about the bot. The bot's token can be fetched using the method [`telers_methods::methods::GetManagedBotToken`](https://docs.rs/telers-methods/latest/telers_methods/methods/get_managed_bot_token/struct.GetManagedBotToken.html).
    pub bot: Box<crate::types::User>,
}
impl ManagedBotCreated {
    /// Creates a new `ManagedBotCreated`.
    ///
    /// # Arguments
    /// * `bot` - Information about the bot. The bot's token can be fetched using the method [`telers_methods::methods::GetManagedBotToken`](https://docs.rs/telers-methods/latest/telers_methods/methods/get_managed_bot_token/struct.GetManagedBotToken.html).
    #[must_use]
    pub fn new<T0: Into<crate::types::User>>(bot: T0) -> Self {
        Self {
            bot: Box::new(bot.into()),
        }
    }

    /// Information about the bot. The bot's token can be fetched using the method [`telers_methods::methods::GetManagedBotToken`](https://docs.rs/telers-methods/latest/telers_methods/methods/get_managed_bot_token/struct.GetManagedBotToken.html).
    #[must_use]
    pub fn bot<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.bot = Box::new(val.into());
        self
    }
}
