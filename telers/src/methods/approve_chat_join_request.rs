use crate::client::Bot;
use serde::Serialize;
/// Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the `can_invite_users` administrator right. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#approvechatjoinrequest>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct ApproveChatJoinRequest {
    /// Unique identifier for the target chat or username of the target channel in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
}
impl ApproveChatJoinRequest {
    /// Creates a new `ApproveChatJoinRequest`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel in the format @username
    /// * `user_id` - Unique identifier of the target user
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<i64>>(
        chat_id: T0,
        user_id: T1,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id: user_id.into(),
        }
    }

    /// Unique identifier for the target chat or username of the target channel in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    /// Unique identifier of the target user
    #[must_use]
    pub fn user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.user_id = val.into();
        self
    }
}
impl super::TelegramMethod for ApproveChatJoinRequest {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("approveChatJoinRequest", self, None)
    }
}
