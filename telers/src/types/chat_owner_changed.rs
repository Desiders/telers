use serde::{Deserialize, Serialize};
/// Describes a service message about an ownership change in the chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatownerchanged>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatOwnerChanged {
    /// The new owner of the chat
    pub new_owner: Box<crate::types::User>,
}
impl ChatOwnerChanged {
    /// Creates a new `ChatOwnerChanged`.
    ///
    /// # Arguments
    /// * `new_owner` - The new owner of the chat
    #[must_use]
    pub fn new<T0: Into<crate::types::User>>(new_owner: T0) -> Self {
        Self {
            new_owner: Box::new(new_owner.into()),
        }
    }

    /// The new owner of the chat
    #[must_use]
    pub fn new_owner<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.new_owner = Box::new(val.into());
        self
    }
}
