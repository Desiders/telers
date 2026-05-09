use serde::{Deserialize, Serialize};
/// The boost was obtained by the creation of Telegram Premium gift codes to boost a chat. Each such code boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboostsourcegiftcode>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatBoostSourceGiftCode {
    /// User for which the gift code was created
    pub user: Box<crate::types::User>,
}
impl ChatBoostSourceGiftCode {
    /// Creates a new `ChatBoostSourceGiftCode`.
    ///
    /// # Arguments
    /// * `user` - User for which the gift code was created
    #[must_use]
    pub fn new<T0: Into<crate::types::User>>(user: T0) -> Self {
        Self {
            user: Box::new(user.into()),
        }
    }

    /// User for which the gift code was created
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.user = Box::new(val.into());
        self
    }
}
