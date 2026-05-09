use crate::client::Bot;
use serde::Serialize;
/// Use this method to set a tag for a regular member in a group or a supergroup. The bot must be an administrator in the chat for this to work and must have the `can_manage_tags` administrator right. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setchatmembertag>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetChatMemberTag {
    /// Unique identifier for the target chat or username of the target supergroup in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// New tag for the member; 0-16 characters, emoji are not allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Box<str>>,
}
impl SetChatMemberTag {
    /// Creates a new `SetChatMemberTag`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup in the format @username
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
            tag: None,
        }
    }

    /// Unique identifier for the target chat or username of the target supergroup in the format @username
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

    /// New tag for the member; 0-16 characters, emoji are not allowed
    #[must_use]
    pub fn tag<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.tag = Some(val.into());
        self
    }

    /// New tag for the member; 0-16 characters, emoji are not allowed
    #[must_use]
    pub fn tag_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.tag = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for SetChatMemberTag {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setChatMemberTag", self, None)
    }
}
