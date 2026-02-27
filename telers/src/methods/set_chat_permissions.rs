use crate::client::Bot;
use serde::Serialize;
/// Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the `can_restrict_members` administrator rights. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setchatpermissions>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetChatPermissions {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: crate::types::ChatIdKind,
    /// A JSON-serialized object for new default chat permissions
    pub permissions: crate::types::ChatPermissions,
    /// Pass `true` if chat permissions are set independently. Otherwise, the `can_send_other_messages` and `can_add_web_page_previews` permissions will imply the `can_send_messages`, `can_send_audios`, `can_send_documents`, `can_send_photos`, `can_send_videos`, `can_send_video_notes`, and `can_send_voice_notes` permissions; the `can_send_polls` permission will imply the `can_send_messages` permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_independent_chat_permissions: Option<bool>,
}
impl SetChatPermissions {
    /// Creates a new `SetChatPermissions`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    /// * `permissions` - A JSON-serialized object for new default chat permissions
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<crate::types::ChatPermissions>>(
        chat_id: T0,
        permissions: T1,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            permissions: permissions.into(),
            use_independent_chat_permissions: None,
        }
    }

    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// A JSON-serialized object for new default chat permissions
    #[must_use]
    pub fn permissions<T: Into<crate::types::ChatPermissions>>(self, val: T) -> Self {
        let mut this = self;
        this.permissions = val.into();
        this
    }

    /// Pass `true` if chat permissions are set independently. Otherwise, the `can_send_other_messages` and `can_add_web_page_previews` permissions will imply the `can_send_messages`, `can_send_audios`, `can_send_documents`, `can_send_photos`, `can_send_videos`, `can_send_video_notes`, and `can_send_voice_notes` permissions; the `can_send_polls` permission will imply the `can_send_messages` permission.
    #[must_use]
    pub fn use_independent_chat_permissions<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.use_independent_chat_permissions = Some(val.into());
        this
    }

    /// Pass `true` if chat permissions are set independently. Otherwise, the `can_send_other_messages` and `can_add_web_page_previews` permissions will imply the `can_send_messages`, `can_send_audios`, `can_send_documents`, `can_send_photos`, `can_send_videos`, `can_send_video_notes`, and `can_send_voice_notes` permissions; the `can_send_polls` permission will imply the `can_send_messages` permission.
    #[must_use]
    pub fn use_independent_chat_permissions_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.use_independent_chat_permissions = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for SetChatPermissions {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setChatPermissions", self, None)
    }
}
