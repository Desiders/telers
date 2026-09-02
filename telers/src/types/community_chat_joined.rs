use serde::{Deserialize, Serialize};
/// Describes a service message about a chat being joined by a user from a community.
/// # Documentation
/// <https://core.telegram.org/bots/api#communitychatjoined>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommunityChatJoined {
    /// The community from which the chat was joined
    pub community: crate::types::Community,
}
impl CommunityChatJoined {
    /// Creates a new `CommunityChatJoined`.
    ///
    /// # Arguments
    /// * `community` - The community from which the chat was joined
    #[must_use]
    pub fn new<T0: Into<crate::types::Community>>(community: T0) -> Self {
        Self {
            community: community.into(),
        }
    }

    /// The community from which the chat was joined
    #[must_use]
    pub fn community<T: Into<crate::types::Community>>(mut self, val: T) -> Self {
        self.community = val.into();
        self
    }
}
