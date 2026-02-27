use serde::{Deserialize, Serialize};
/// This object represents a service message about a forum topic reopened in the chat. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#forumtopicreopened>
#[derive(Clone, Debug, Serialize, Deserialize)]
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
