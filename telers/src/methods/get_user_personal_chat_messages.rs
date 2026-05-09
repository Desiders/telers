use crate::client::Bot;
use serde::Serialize;
/// Use this method to get the last messages from the personal chat (i.e., the chat currently added to their profile) of a given user. On success, an array of Message objects is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#getuserpersonalchatmessages>
/// # Returns
/// - `Box<[crate::types::Message]>`
#[derive(Clone, Debug, Serialize)]
pub struct GetUserPersonalChatMessages {
    /// Unique identifier for the target user
    pub user_id: i64,
    /// The maximum number of messages to return; 1-20
    pub limit: u8,
}
impl GetUserPersonalChatMessages {
    /// Creates a new `GetUserPersonalChatMessages`.
    ///
    /// # Arguments
    /// * `user_id` - Unique identifier for the target user
    /// * `limit` - The maximum number of messages to return; 1-20
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<u8>>(user_id: T0, limit: T1) -> Self {
        Self {
            user_id: user_id.into(),
            limit: limit.into(),
        }
    }

    /// Unique identifier for the target user
    #[must_use]
    pub fn user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.user_id = val.into();
        self
    }

    /// The maximum number of messages to return; 1-20
    #[must_use]
    pub fn limit<T: Into<u8>>(mut self, val: T) -> Self {
        self.limit = val.into();
        self
    }
}
impl super::TelegramMethod for GetUserPersonalChatMessages {
    type Method = Self;
    type Return = Box<[crate::types::Message]>;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getUserPersonalChatMessages", self, None)
    }
}
