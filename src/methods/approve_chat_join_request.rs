use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to approve a chat join request. The bot must be an administrator in the chat for this to work and must have the `can_invite_users` administrator right.
/// # Documentation
/// <https://core.telegram.org/bots/api#approvechatjoinrequest>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct ApproveChatJoinRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
}

impl ApproveChatJoinRequest {
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

impl TelegramMethod for ApproveChatJoinRequest {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("approveChatJoinRequest", self, None)
    }
}

impl AsRef<ApproveChatJoinRequest> for ApproveChatJoinRequest {
    fn as_ref(&self) -> &Self {
        self
    }
}
