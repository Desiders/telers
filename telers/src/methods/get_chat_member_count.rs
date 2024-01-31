use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;

/// Use this method to get the number of members in a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#getchatmembercount>
/// # Returns
/// Returns `i64` on success
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct GetChatMemberCount {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
}

impl GetChatMemberCount {
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

impl TelegramMethod for GetChatMemberCount {
    type Method = Self;
    type Return = i64;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("getChatMemberCount", self, None)
    }
}

impl AsRef<GetChatMemberCount> for GetChatMemberCount {
    fn as_ref(&self) -> &Self {
        self
    }
}
