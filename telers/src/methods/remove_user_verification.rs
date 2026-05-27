use crate::client::Bot;
use serde::Serialize;
/// Removes verification from a user who is currently verified on behalf of the organization represented by the bot. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#removeuserverification>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct RemoveUserVerification {
    /// Unique identifier of the target user
    pub user_id: i64,
}
impl RemoveUserVerification {
    /// Creates a new `RemoveUserVerification`.
    ///
    /// # Arguments
    /// * `user_id` - Unique identifier of the target user
    #[must_use]
    pub fn new<T0: Into<i64>>(user_id: T0) -> Self {
        Self {
            user_id: user_id.into(),
        }
    }

    /// Unique identifier of the target user
    #[must_use]
    pub fn user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.user_id = val.into();
        self
    }
}
impl super::TelegramMethod for RemoveUserVerification {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("removeUserVerification", self, None)
    }
}
