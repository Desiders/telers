use super::{InlineKeyboardMarkup, InputMessageContent, MessageEntity};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a link to a file stored on the Telegram servers. By default, this file will be sent by the user with an optional caption. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the file.
/// # Notes
/// This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultcacheddocument>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultCachedDocument {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A Title for the result
    pub title: String,
    /// A valid file identifier for the file
    pub document_file_id: String,
    /// Caption of the document to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the document caption. See [`formatting options`](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Short description of the result
    pub description: Option<String>,
    /// Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the file
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedDocument {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        document_file_id: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            document_file_id: document_file_id.into(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            description: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    #[must_use]
    pub fn id(self, val: impl Into<String>) -> Self {
        Self {
            id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn title(self, val: impl Into<String>) -> Self {
        Self {
            title: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn document_file_id(self, val: impl Into<String>) -> Self {
        Self {
            document_file_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn caption(self, val: impl Into<String>) -> Self {
        Self {
            caption: Some(val.into()),
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
    pub fn caption_entity(self, val: MessageEntity) -> Self {
        Self {
            caption_entities: Some(
                self.caption_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(Some(val))
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn caption_entities(self, val: impl IntoIterator<Item = MessageEntity>) -> Self {
        Self {
            caption_entities: Some(
                self.caption_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn description(self, val: impl Into<String>) -> Self {
        Self {
            description: Some(val.into()),
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

    #[must_use]
    pub fn input_message_content(self, val: impl Into<InputMessageContent>) -> Self {
        Self {
            input_message_content: Some(val.into()),
            ..self
        }
    }
}

impl InlineQueryResultCachedDocument {
    #[must_use]
    pub fn caption_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            caption: val.map(Into::into),
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
    pub fn caption_entities_option(
        self,
        val: Option<impl IntoIterator<Item = MessageEntity>>,
    ) -> Self {
        Self {
            caption_entities: val.map(|val| {
                self.caption_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect()
            }),
            ..self
        }
    }
}
