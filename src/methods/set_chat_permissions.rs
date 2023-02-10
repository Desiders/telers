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
}

impl SetChatPermissions {
    #[must_use]
    pub fn new<T: Into<ChatIdKind>>(chat_id: T, permissions: ChatPermissions) -> Self {
        Self {
            chat_id: chat_id.into(),
            permissions,
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    #[must_use]
    pub fn permissions(mut self, val: ChatPermissions) -> Self {
        self.permissions = val;
        self
    }
}

impl TelegramMethod for SetChatPermissions {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setChatPermissions", self, None)
    }
}
