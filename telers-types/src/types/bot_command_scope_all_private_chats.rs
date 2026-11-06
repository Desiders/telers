/// Represents the scope of bot commands, covering all private chats.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscopeallprivatechats>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct BotCommandScopeAllPrivateChats {}
impl BotCommandScopeAllPrivateChats {
    /// Creates a new `BotCommandScopeAllPrivateChats`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for BotCommandScopeAllPrivateChats {
    fn default() -> Self {
        Self::new()
    }
}
