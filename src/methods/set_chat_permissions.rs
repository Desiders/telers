use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, ChatPermissions},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the `can_restrict_members` administrator rights.
/// # Documentation
/// <https://core.telegram.org/bots/api#setchatpermissions>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SetChatPermissions {
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatIdKind,
    /// A JSON-serialized object for new default chat permissions
    pub permissions: ChatPermissions,
    /// Pass `True` if chat permissions are set independently. Otherwise, the `can_send_other_messages` and `can_add_web_page_previews` permissions will imply the `can_send_messages`, `can_send_audios`, `can_send_documents`, `can_send_photos`, `can_send_videos`, `can_send_video_notes`, and `can_send_voice_notes` permissions; the `can_send_polls` permission will imply the `can_send_messages` permission.
    pub use_independent_chat_permissions: Option<bool>,
}

impl SetChatPermissions {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, permissions: ChatPermissions) -> Self {
        Self {
            chat_id: chat_id.into(),
            permissions,
            use_independent_chat_permissions: None,
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn permissions(self, val: ChatPermissions) -> Self {
        Self {
            permissions: val,
            ..self
        }
    }

    #[must_use]
    pub fn use_independent_chat_permissions(self, val: bool) -> Self {
        Self {
            use_independent_chat_permissions: Some(val),
            ..self
        }
    }
}

impl SetChatPermissions {
    #[must_use]
    pub fn use_independent_chat_permissions_option(self, val: Option<bool>) -> Self {
        Self {
            use_independent_chat_permissions: val,
            ..self
        }
    }
}

impl TelegramMethod for SetChatPermissions {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setChatPermissions", self, None)
    }
}
