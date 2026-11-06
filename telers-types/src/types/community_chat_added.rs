/// Describes a service message about a chat or a bot being added to a community.
/// # Documentation
/// <https://core.telegram.org/bots/api#communitychatadded>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct CommunityChatAdded {
    /// The new community to which the chat or the bot belongs
    pub community: crate::types::Community,
}
impl CommunityChatAdded {
    /// Creates a new `CommunityChatAdded`.
    ///
    /// # Arguments
    /// * `community` - The new community to which the chat or the bot belongs
    #[must_use]
    pub fn new<T0: Into<crate::types::Community>>(community: T0) -> Self {
        Self {
            community: community.into(),
        }
    }

    /// The new community to which the chat or the bot belongs
    #[must_use]
    pub fn community<T: Into<crate::types::Community>>(mut self, val: T) -> Self {
        self.community = val.into();
        self
    }
}
