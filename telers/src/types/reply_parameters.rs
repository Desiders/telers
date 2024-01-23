use super::{ChatIdKind, MessageEntity};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes reply parameters for the message that is being sent.
/// # Documentation
/// <https://core.telegram.org/bots/api#replyparameters>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReplyParameters {
    /// Identifier of the message that will be replied to in the current chat, or in the chat chat_id if it is specified
    pub message_id: i64,
    /// If the message to be replied to is from a different chat, unique identifier for the chat or username of the channel (in the format `@channelusername`)
    pub chat_id: Option<ChatIdKind>,
    /// Pass `true` if the message should be sent even if the specified message to be replied to is not found; can be used only for replies in the same chat and forum topic.
    pub allow_sending_without_reply: Option<bool>,
    /// Quoted part of the message to be replied to; 0-1024 characters after entities parsing. The quote must be an exact substring of the message to be replied to, including `bold`, `italic, `underline`, `strikethrough`, `spoiler`, and `custom_emoji` entities. The message will fail to send if the quote isn't found in the original message.
    pub quote: Option<String>,
    /// Mode for parsing entities in the quote. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub quote_parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the quote. It can be specified instead of `quote_parse_mode`
    pub quote_entities: Option<Vec<MessageEntity>>,
    /// Position of the quote in the original message in UTF-16 code units
    pub quote_position: Option<u16>,
}

impl ReplyParameters {
    #[must_use]
    pub fn new(message_id: i64) -> Self {
        Self {
            message_id,
            chat_id: None,
            allow_sending_without_reply: None,
            quote: None,
            quote_parse_mode: None,
            quote_entities: None,
            quote_position: None,
        }
    }

    #[must_use]
    pub fn message_id(self, val: i64) -> Self {
        Self {
            message_id: val,
            ..self
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
    pub fn allow_sending_without_reply(self, val: bool) -> Self {
        Self {
            allow_sending_without_reply: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn quote(self, val: impl Into<String>) -> Self {
        Self {
            quote: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn quote_parse_mode(self, val: impl Into<String>) -> Self {
        Self {
            quote_parse_mode: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn quote_entity(self, val: MessageEntity) -> Self {
        Self {
            quote_entities: Some(
                self.quote_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(Some(val))
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn quote_entities(self, val: impl IntoIterator<Item = MessageEntity>) -> Self {
        Self {
            quote_entities: Some(
                self.quote_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn quote_position(self, val: u16) -> Self {
        Self {
            quote_position: Some(val),
            ..self
        }
    }
}

impl ReplyParameters {
    #[must_use]
    pub fn chat_id_option(self, val: Option<impl Into<ChatIdKind>>) -> Self {
        Self {
            chat_id: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn allow_sending_without_reply_option(self, val: Option<bool>) -> Self {
        Self {
            allow_sending_without_reply: val,
            ..self
        }
    }

    #[must_use]
    pub fn quote_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            quote: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn quote_parse_mode_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            quote_parse_mode: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn quote_entities_option(
        self,
        val: Option<impl IntoIterator<Item = MessageEntity>>,
    ) -> Self {
        Self {
            quote_entities: val.map(|val| {
                self.quote_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect()
            }),
            ..self
        }
    }

    #[must_use]
    pub fn quote_position_option(self, val: Option<u16>) -> Self {
        Self {
            quote_position: val,
            ..self
        }
    }
}
