/// This object represents the bot's description.
/// # Documentation
/// <https://core.telegram.org/bots/api#botdescription>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
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
    pub fn description<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.description = val.into();
        self
    }
}
