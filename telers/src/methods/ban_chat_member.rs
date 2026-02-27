use crate::client::Bot;
use serde::Serialize;
/// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless unbanned first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#banchatmember>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct BanChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Date when the user will be unbanned; Unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever. Applied for supergroups and channels only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
    /// Pass `true` to delete all messages from the chat for the user that is being removed. If `false`, the user will be able to see messages in the group that were sent before the user was removed. Always `true` for supergroups and channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_messages: Option<bool>,
}
impl BanChatMember {
    /// Creates a new `BanChatMember`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target group or username of the target supergroup or channel (in the format @channelusername)
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
            until_date: None,
            revoke_messages: None,
        }
    }

    /// Unique identifier for the target group or username of the target supergroup or channel (in the format @channelusername)
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

    /// Date when the user will be unbanned; Unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever. Applied for supergroups and channels only.
    #[must_use]
    pub fn until_date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.until_date = Some(val.into());
        this
    }

    /// Date when the user will be unbanned; Unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever. Applied for supergroups and channels only.
    #[must_use]
    pub fn until_date_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.until_date = val.map(Into::into);
        this
    }

    /// Pass `true` to delete all messages from the chat for the user that is being removed. If `false`, the user will be able to see messages in the group that were sent before the user was removed. Always `true` for supergroups and channels.
    #[must_use]
    pub fn revoke_messages<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.revoke_messages = Some(val.into());
        this
    }

    /// Pass `true` to delete all messages from the chat for the user that is being removed. If `false`, the user will be able to see messages in the group that were sent before the user was removed. Always `true` for supergroups and channels.
    #[must_use]
    pub fn revoke_messages_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.revoke_messages = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for BanChatMember {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("banChatMember", self, None)
    }
}
