/// Represents the scope of bot commands, covering all group and supergroup chats.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscopeallgroupchats>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct BotCommandScopeAllGroupChats {}
impl BotCommandScopeAllGroupChats {
    /// Creates a new `BotCommandScopeAllGroupChats`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for BotCommandScopeAllGroupChats {
    fn default() -> Self {
        Self::new()
    }
}
