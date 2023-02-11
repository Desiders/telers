use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, InlineKeyboardMarkup, Poll},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to stop a poll which was sent by the bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#stoppoll>
/// # Returns
/// On success, the stopped [`Poll`] is returned
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct StopPoll {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Identifier of the original message with the poll
    pub message_id: i64,
    /// A JSON-serialized object for a new message [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl StopPoll {
    #[must_use]
    pub fn new<T: Into<ChatIdKind>>(chat_id: T, message_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id,
            reply_markup: None,
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    #[must_use]
    pub fn message_id(mut self, val: i64) -> Self {
        self.message_id = val;
        self
    }

    #[must_use]
    pub fn reply_markup<T: Into<InlineKeyboardMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }
}

impl TelegramMethod for StopPoll {
    type Method = Self;
    type Return = Poll;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("stopPoll", self, None)
    }
}
