use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, InlineKeyboardMarkup, MessageEntity, MessageOrTrue},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to edit captions of messages.
/// # Documentation
/// <https://core.telegram.org/bots/api#editmessagecaption>
/// # Returns
/// On success, if the edited message is not an inline message, the edited [`crate::types::Message`] is returned, otherwise `True` is returned
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct EditMessageCaption {
    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: Option<ChatIdKind>,
    /// Required if `inline_message_id` is not specified. Identifier of the message to edit
    pub message_thread_id: Option<i64>,
    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// New caption of the message, 0-1024 characters after entities parsing
    pub caption: String,
    /// Mode for parsing entities in the message caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of `parse_mode`
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageCaption {
    #[must_use]
    pub fn new<T: Into<String>>(caption: T) -> Self {
        Self {
            chat_id: None,
            message_thread_id: None,
            inline_message_id: None,
            caption: caption.into(),
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = Some(val.into());
        self
    }

    #[must_use]
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }

    #[must_use]
    pub fn inline_message_id<T: Into<String>>(mut self, val: T) -> Self {
        self.inline_message_id = Some(val.into());
        self
    }

    #[must_use]
    pub fn caption<T: Into<String>>(mut self, val: T) -> Self {
        self.caption = val.into();
        self
    }

    #[must_use]
    pub fn parse_mode<T: Into<String>>(mut self, val: T) -> Self {
        self.parse_mode = Some(val.into());
        self
    }

    #[must_use]
    pub fn caption_entities<T: Into<Vec<MessageEntity>>>(mut self, val: T) -> Self {
        self.caption_entities = Some(val.into());
        self
    }

    #[must_use]
    pub fn reply_markup<T: Into<InlineKeyboardMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }
}

impl EditMessageCaption {
    #[must_use]
    pub fn chat_id_some<T: Into<ChatIdKind>>(mut self, val: Option<T>) -> Self {
        self.chat_id = val.map(Into::into);
        self
    }

    #[must_use]
    pub fn message_thread_id_some(mut self, val: Option<i64>) -> Self {
        self.message_thread_id = val;
        self
    }

    #[must_use]
    pub fn inline_message_id_some<T: Into<String>>(mut self, val: Option<T>) -> Self {
        self.inline_message_id = val.map(Into::into);
        self
    }

    #[must_use]
    pub fn caption_some<T: Into<String>>(mut self, val: Option<T>) -> Self {
        self.caption = val.map(Into::into).unwrap_or_default();
        self
    }

    #[must_use]
    pub fn parse_mode_some<T: Into<String>>(mut self, val: Option<T>) -> Self {
        self.parse_mode = val.map(Into::into);
        self
    }

    #[must_use]
    pub fn caption_entities_some<T: Into<Vec<MessageEntity>>>(mut self, val: Option<T>) -> Self {
        self.caption_entities = val.map(Into::into);
        self
    }

    #[must_use]
    pub fn reply_markup_some<T: Into<InlineKeyboardMarkup>>(mut self, val: Option<T>) -> Self {
        self.reply_markup = val.map(Into::into);
        self
    }
}

impl TelegramMethod for EditMessageCaption {
    type Method = Self;
    type Return = MessageOrTrue;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("editMessageCaption", self, None)
    }
}
