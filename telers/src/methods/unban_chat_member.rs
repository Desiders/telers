use crate::client::Bot;
use serde::Serialize;
/// Use this method to unban a previously banned user in a supergroup or channel. The user will not return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. By default, this method guarantees that after the call the user is not a member of the chat, but will be able to join it. So if the user is a member of the chat they will also be removed from the chat. If you don't want this, use the parameter `only_if_banned`. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#unbanchatmember>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct UnbanChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Do nothing if the user is not banned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_if_banned: Option<bool>,
}
impl UnbanChatMember {
    /// Creates a new `UnbanChatMember`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target group or username of the target supergroup or channel in the format @username
    /// * `user_id` - Unique identifier of the target user
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<i64>>(
        chat_id: T0,
        user_id: T1,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id: user_id.into(),
            only_if_banned: None,
        }
    }

    /// Unique identifier for the target group or username of the target supergroup or channel in the format @username
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

    /// Do nothing if the user is not banned
    #[must_use]
    pub fn only_if_banned<T: Into<bool>>(mut self, val: T) -> Self {
        self.only_if_banned = Some(val.into());
        self
    }

    /// Do nothing if the user is not banned
    #[must_use]
    pub fn only_if_banned_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.only_if_banned = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for UnbanChatMember {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("unbanChatMember", self, None)
    }
}
