use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to ban a user in a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the chat on their own using invite links, etc., unless [`unbanned`](crate::methods::UnbanChatMember) first. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// # Documentation
/// <https://core.telegram.org/bots/api#banchatmember>
/// # Returns
/// Returns `true` on success
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct BanChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Date when the user will be unbanned; Unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever. Applied for supergroups and channels only.
    pub until_date: Option<i64>,
    /// Pass `true` to delete all messages from the chat for the user that is being removed. If `False`, the user will be able to see messages in the group that were sent before the user was removed. Always `true` for supergroups and channels.
    pub revoke_messages: Option<bool>,
}

impl BanChatMember {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, user_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
            until_date: None,
            revoke_messages: None,
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
    pub fn user_id(self, val: i64) -> Self {
        Self {
            user_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn until_date(self, val: i64) -> Self {
        Self {
            until_date: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn revoke_messages(self, val: bool) -> Self {
        Self {
            revoke_messages: Some(val),
            ..self
        }
    }
}

impl BanChatMember {
    #[must_use]
    pub fn until_date_option(self, val: Option<i64>) -> Self {
        Self {
            until_date: val,
            ..self
        }
    }

    #[must_use]
    pub fn revoke_messages_option(self, val: Option<bool>) -> Self {
        Self {
            revoke_messages: val,
            ..self
        }
    }
}

impl TelegramMethod for BanChatMember {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("banChatMember", self, None)
    }
}

impl AsRef<BanChatMember> for BanChatMember {
    fn as_ref(&self) -> &Self {
        self
    }
}
