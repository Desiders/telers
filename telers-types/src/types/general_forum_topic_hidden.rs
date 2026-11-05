/// This object represents a service message about General forum topic hidden in the chat. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#generalforumtopichidden>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct GeneralForumTopicHidden {}
impl GeneralForumTopicHidden {
    /// Creates a new `GeneralForumTopicHidden`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for GeneralForumTopicHidden {
    fn default() -> Self {
        Self::new()
    }
}
