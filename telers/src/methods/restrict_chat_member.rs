use crate::client::Bot;
use serde::Serialize;
/// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass `true` for all permissions to lift restrictions from a user. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#restrictchatmember>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct RestrictChatMember {
    /// Unique identifier for the target chat or username of the target supergroup in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// A JSON-serialized object for new user permissions
    pub permissions: crate::types::ChatPermissions,
    /// Pass `true` if chat permissions are set independently. Otherwise, the `can_send_other_messages` and `can_add_web_page_previews` permissions will imply the `can_send_messages`, `can_send_audios`, `can_send_documents`, `can_send_photos`, `can_send_videos`, `can_send_video_notes`, and `can_send_voice_notes` permissions; the `can_send_polls` permission will imply the `can_send_messages` permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_independent_chat_permissions: Option<bool>,
    /// Date when restrictions will be lifted for the user; Unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}
impl RestrictChatMember {
    /// Creates a new `RestrictChatMember`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup in the format @username
    /// * `user_id` - Unique identifier of the target user
    /// * `permissions` - A JSON-serialized object for new user permissions
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<crate::types::ChatIdKind>,
        T1: Into<i64>,
        T2: Into<crate::types::ChatPermissions>,
    >(
        chat_id: T0,
        user_id: T1,
        permissions: T2,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id: user_id.into(),
            permissions: permissions.into(),
            use_independent_chat_permissions: None,
            until_date: None,
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

    /// A JSON-serialized object for new user permissions
    #[must_use]
    pub fn permissions<T: Into<crate::types::ChatPermissions>>(mut self, val: T) -> Self {
        self.permissions = val.into();
        self
    }

    /// Pass `true` if chat permissions are set independently. Otherwise, the `can_send_other_messages` and `can_add_web_page_previews` permissions will imply the `can_send_messages`, `can_send_audios`, `can_send_documents`, `can_send_photos`, `can_send_videos`, `can_send_video_notes`, and `can_send_voice_notes` permissions; the `can_send_polls` permission will imply the `can_send_messages` permission.
    #[must_use]
    pub fn use_independent_chat_permissions<T: Into<bool>>(mut self, val: T) -> Self {
        self.use_independent_chat_permissions = Some(val.into());
        self
    }

    /// Pass `true` if chat permissions are set independently. Otherwise, the `can_send_other_messages` and `can_add_web_page_previews` permissions will imply the `can_send_messages`, `can_send_audios`, `can_send_documents`, `can_send_photos`, `can_send_videos`, `can_send_video_notes`, and `can_send_voice_notes` permissions; the `can_send_polls` permission will imply the `can_send_messages` permission.
    #[must_use]
    pub fn use_independent_chat_permissions_option<T: Into<bool>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.use_independent_chat_permissions = val.map(Into::into);
        self
    }

    /// Date when restrictions will be lifted for the user; Unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever
    #[must_use]
    pub fn until_date<T: Into<i64>>(mut self, val: T) -> Self {
        self.until_date = Some(val.into());
        self
    }

    /// Date when restrictions will be lifted for the user; Unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever
    #[must_use]
    pub fn until_date_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.until_date = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for RestrictChatMember {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("restrictChatMember", self, None)
    }
}
