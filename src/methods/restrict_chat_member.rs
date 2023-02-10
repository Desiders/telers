use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, ChatPermissions},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass `True` for all permissions to lift restrictions from a user.
/// # Documentation
/// <https://core.telegram.org/bots/api#restrictchatmember>
/// # Returns
/// Returns `True` on success.
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct RestrictChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// A JSON-serialized object for new user permissions
    pub permissions: ChatPermissions,
    /// Date when restrictions will be lifted for the user, unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever
    pub until_date: Option<i64>,
}

impl RestrictChatMember {
    #[must_use]
    pub fn new<T: Into<ChatIdKind>>(
        chat_id: T,
        user_id: i64,
        permissions: ChatPermissions,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
            permissions,
            until_date: None,
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    #[must_use]
    pub fn user_id(mut self, val: i64) -> Self {
        self.user_id = val;
        self
    }

    #[must_use]
    pub fn permissions(mut self, val: ChatPermissions) -> Self {
        self.permissions = val;
        self
    }

    #[must_use]
    pub fn until_date(mut self, val: i64) -> Self {
        self.until_date = Some(val);
        self
    }
}

impl TelegramMethod for RestrictChatMember {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("restrictChatMember", self, None)
    }
}
