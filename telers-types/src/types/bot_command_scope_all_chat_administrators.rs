/// Represents the scope of bot commands, covering all group and supergroup chat administrators.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscopeallchatadministrators>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct BotCommandScopeAllChatAdministrators {}
impl BotCommandScopeAllChatAdministrators {
    /// Creates a new `BotCommandScopeAllChatAdministrators`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for BotCommandScopeAllChatAdministrators {
    fn default() -> Self {
        Self::new()
    }
}
