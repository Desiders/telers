use serde::{Deserialize, Serialize};
/// The boost was obtained by subscribing to Telegram Premium or by gifting a Telegram Premium subscription to another user.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboostsourcepremium>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatBoostSourcePremium {
    /// User that boosted the chat
    pub user: Box<crate::types::User>,
}
impl ChatBoostSourcePremium {
    /// Creates a new `ChatBoostSourcePremium`.
    ///
    /// # Arguments
    /// * `user` - User that boosted the chat
    #[must_use]
    pub fn new<T0: Into<crate::types::User>>(user: T0) -> Self {
        Self {
            user: Box::new(user.into()),
        }
    }

    /// User that boosted the chat
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.user = Box::new(val.into());
        self
    }
}
