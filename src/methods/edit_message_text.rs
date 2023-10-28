use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, InlineKeyboardMarkup, MessageEntity, MessageOrTrue},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to edit text and [game](https://core.telegram.org/bots/api#games) messages.
/// # Documentation
/// <https://core.telegram.org/bots/api#editmessagetext>
/// # Returns
/// On success, if the edited message is not an inline message, the edited [`crate::types::Message`] is returned, otherwise `True` is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct EditMessageText {
    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: Option<ChatIdKind>,
    /// Required if `inline_message_id` is not specified. Identifier of the message to edit
    pub message_thread_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// New text of the message, 1-4096 characters after entities parsing
    pub text: String,
    /// Mode for parsing entities in the new caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in message text, which can be specified instead of `parse_mode`
    pub entities: Option<Vec<MessageEntity>>,
    /// Disables link previews for links in this message
    pub disable_web_page_preview: Option<bool>,
    /// A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageText {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            chat_id: None,
            message_thread_id: None,
            inline_message_id: None,
            text: text.into(),
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn message_thread_id(self, val: i64) -> Self {
        Self {
            message_thread_id: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn inline_message_id(self, val: impl Into<String>) -> Self {
        Self {
            inline_message_id: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn text(self, val: impl Into<String>) -> Self {
        Self {
            text: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn parse_mode(self, val: impl Into<String>) -> Self {
        Self {
            parse_mode: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn entities(self, val: impl IntoIterator<Item = MessageEntity>) -> Self {
        Self {
            entities: Some(
                self.entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn disable_web_page_preview(self, val: bool) -> Self {
        Self {
            disable_web_page_preview: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup(self, val: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(val.into()),
            ..self
        }
    }
}

impl EditMessageText {
    #[must_use]
    pub fn chat_id_option(self, val: Option<impl Into<ChatIdKind>>) -> Self {
        Self {
            chat_id: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn message_thread_id_option(self, val: Option<i64>) -> Self {
        Self {
            message_thread_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn inline_message_id_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            inline_message_id: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn parse_mode_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            parse_mode: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn entities_option(self, val: Option<impl IntoIterator<Item = MessageEntity>>) -> Self {
        Self {
            entities: val.map(|val| {
                self.entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect()
            }),
            ..self
        }
    }

    #[must_use]
    pub fn disable_web_page_preview_option(self, val: Option<bool>) -> Self {
        Self {
            disable_web_page_preview: val,
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup_option(self, val: Option<impl Into<InlineKeyboardMarkup>>) -> Self {
        Self {
            reply_markup: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for EditMessageText {
    type Method = Self;
    type Return = MessageOrTrue;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("editMessageText", self, None)
    }
}

impl AsRef<EditMessageText> for EditMessageText {
    fn as_ref(&self) -> &Self {
        self
    }
}
