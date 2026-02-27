use serde::{Deserialize, Serialize};
/// This object represents a service message about General forum topic hidden in the chat. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#generalforumtopichidden>
#[derive(Clone, Debug, Serialize, Deserialize)]
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
