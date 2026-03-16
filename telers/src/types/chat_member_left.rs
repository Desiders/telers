use serde::{Deserialize, Serialize};
/// Represents a chat member that isn't currently a member of the chat, but may join it themselves.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatmemberleft>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMemberLeft {
    /// Information about the user
    pub user: Box<crate::types::User>,
}
impl ChatMemberLeft {
    /// Creates a new `ChatMemberLeft`.
    ///
    /// # Arguments
    /// * `user` - Information about the user
    #[must_use]
    pub fn new<T0: Into<crate::types::User>>(user: T0) -> Self {
        Self {
            user: Box::new(user.into()),
        }
    }

    /// Information about the user
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.user = Box::new(val.into());
        this
    }
}
