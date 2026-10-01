/// Represents the default scope of bot commands. Default commands are used if no commands with a narrower scope are specified for the user.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscopedefault>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct BotCommandScopeDefault {}
impl BotCommandScopeDefault {
    /// Creates a new `BotCommandScopeDefault`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for BotCommandScopeDefault {
    fn default() -> Self {
        Self::new()
    }
}
