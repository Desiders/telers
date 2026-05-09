use crate::client::Bot;
use serde::Serialize;
/// Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the `can_invite_users` administrator right. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#declinechatjoinrequest>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct DeclineChatJoinRequest {
    /// Unique identifier for the target chat or username of the target channel in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
}
impl DeclineChatJoinRequest {
    /// Creates a new `DeclineChatJoinRequest`.
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
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// Unique identifier of the target user
    #[must_use]
    pub fn user_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.user_id = val.into();
        this
    }
}
impl super::TelegramMethod for DeclineChatJoinRequest {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("declineChatJoinRequest", self, None)
    }
}
