use serde::{Deserialize, Serialize};
/// Describes a service message about a chat being removed from a community. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#communitychatremoved>
#[derive(Clone, Debug, Serialize, Deserialize)]
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
