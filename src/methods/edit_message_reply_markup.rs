use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, InlineKeyboardMarkup, MessageOrTrue},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to edit only the reply markup of messages.
/// # Documentation
/// <https://core.telegram.org/bots/api#editmessagereplymarkup>
/// # Returns
/// On success, if the edited message is not an inline message, the edited [`crate::types::Message`] is returned, otherwise `True` is returned
#[skip_serializing_none]
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct EditMessageReplyMarkup {
    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: Option<ChatIdKind>,
    /// Required if `inline_message_id` is not specified. Identifier of the message to edit
    pub message_id: Option<i64>,
    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageReplyMarkup {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = Some(val.into());
        self
    }

    #[must_use]
    pub fn message_id(mut self, val: i64) -> Self {
        self.message_id = Some(val);
        self
    }

    #[must_use]
    pub fn inline_message_id<T: Into<String>>(mut self, val: T) -> Self {
        self.inline_message_id = Some(val.into());
        self
    }

    #[must_use]
    pub fn reply_markup<T: Into<InlineKeyboardMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }
}

impl TelegramMethod for EditMessageReplyMarkup {
    type Method = Self;
    type Return = MessageOrTrue;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("editMessageReplyMarkup", self, None)
    }
}