/// Describes a service message about a chat or a bot being removed from a community. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#communitychatremoved>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct CommunityChatRemoved {}
impl CommunityChatRemoved {
    /// Creates a new `CommunityChatRemoved`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for CommunityChatRemoved {
    fn default() -> Self {
        Self::new()
    }
}
