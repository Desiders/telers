/// Represents a menu button, which opens the bot's list of commands.
/// # Documentation
/// <https://core.telegram.org/bots/api#menubuttoncommands>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct MenuButtonCommands {}
impl MenuButtonCommands {
    /// Creates a new `MenuButtonCommands`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for MenuButtonCommands {
    fn default() -> Self {
        Self::new()
    }
}
