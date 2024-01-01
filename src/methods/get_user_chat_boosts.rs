use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, UserChatBoosts},
};

use serde::Serialize;

/// Use this method to get the list of boosts added to a chat by a user. Requires administrator rights in the chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#getuserchatboosts>
/// # Returns
/// Returns a [`UserChatBoosts`] object.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct GetUserChatBoosts {
    /// Unique identifier for the chat or username of the channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
}

impl GetUserChatBoosts {
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

impl TelegramMethod for GetUserChatBoosts {
    type Method = Self;
    type Return = UserChatBoosts;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("getUserChatBoosts", self, None)
    }
}

impl AsRef<GetUserChatBoosts> for GetUserChatBoosts {
    fn as_ref(&self) -> &Self {
        self
    }
}
