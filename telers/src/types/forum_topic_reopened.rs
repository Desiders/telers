/// This object represents a service message about a forum topic reopened in the chat. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#forumtopicreopened>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct ForumTopicReopened {}
impl ForumTopicReopened {
    /// Creates a new `ForumTopicReopened`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for ForumTopicReopened {
    fn default() -> Self {
        Self::new()
    }
}
