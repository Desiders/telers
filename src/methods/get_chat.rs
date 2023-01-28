use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::{ChatIdKind, Chat}};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to get up to date information about the chat (current name of the user for one-on-one conversations, current username of a user, group or channel, etc.).
/// # Documentation
/// <https://core.telegram.org/bots/api#getchat>
/// # Returns
/// Returns a [`Chat`] object on success.
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct GetChat {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
}

impl GetChat {
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

impl TelegramMethod for GetChat {
    type Method = Self;
    type Return = Chat;

    fn build_request(&self, _: &Bot) -> Request<Self::Method> {
        Request::new("getChat", self, None)
    }
}
