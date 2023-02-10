use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method for your bot to leave a group, supergroup or channel.
/// # Documentation
/// <https://core.telegram.org/bots/api#leavechat>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct LeaveChat {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
}

impl LeaveChat {
    #[must_use]
    pub fn new<T: Into<ChatIdKind>>(chat_id: T) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }
}

impl TelegramMethod for LeaveChat {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("leaveChat", self, None)
    }
}
