/// This object represents a service message about General forum topic unhidden in the chat. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#generalforumtopicunhidden>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct GeneralForumTopicUnhidden {}
impl GeneralForumTopicUnhidden {
    /// Creates a new `GeneralForumTopicUnhidden`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for GeneralForumTopicUnhidden {
    fn default() -> Self {
        Self::new()
    }
}
