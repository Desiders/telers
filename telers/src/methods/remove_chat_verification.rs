use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Removes verification from a chat who is currently verified [on behalf of the organization](https://telegram.org/verify#third-party-verification) represented by the bot
/// # Documentation
/// <https://core.telegram.org/bots/api#removechatverification>
/// # Returns
/// On success, `true` is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct RemoveChatVerification {
    /// Unique identifier for the target chat or username of the target channel (in the format `@supergroupusername`)
    pub chat_id: ChatIdKind,
}

impl RemoveChatVerification {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: val.into(),
        }
    }
}

impl TelegramMethod for RemoveChatVerification {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("removeChatVerification", self, None)
    }
}

impl AsRef<RemoveChatVerification> for RemoveChatVerification {
    fn as_ref(&self) -> &Self {
        self
    }
}
