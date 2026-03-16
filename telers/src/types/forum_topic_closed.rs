use serde::{Deserialize, Serialize};
/// This object represents a service message about a forum topic closed in the chat. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#forumtopicclosed>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForumTopicClosed {}
impl ForumTopicClosed {
    /// Creates a new `ForumTopicClosed`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for ForumTopicClosed {
    fn default() -> Self {
        Self::new()
    }
}
