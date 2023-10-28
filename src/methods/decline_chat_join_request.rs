use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to decline a chat join request. The bot must be an administrator in the chat for this to work and must have the `can_invite_users` administrator right.
/// # Documentation
/// <https://core.telegram.org/bots/api#declinechatjoinrequest>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct DeclineChatJoinRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
}

impl DeclineChatJoinRequest {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, user_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id,
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
}

impl TelegramMethod for DeclineChatJoinRequest {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("declineChatJoinRequest", self, None)
    }
}

impl AsRef<DeclineChatJoinRequest> for DeclineChatJoinRequest {
    fn as_ref(&self) -> &Self {
        self
    }
}
