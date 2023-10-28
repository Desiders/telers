use super::{InlineKeyboardMarkup, InputMessageContent, MessageEntity};

use crate::enums::InlineQueryResultType;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a link to an MP3 audio file. By default, this audio file will be sent by the user. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the audio.
/// # Notes
/// This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultaudio>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultAudio {
    /// Type of the result, must be *audio*
    #[serde(rename = "type", default = "audio")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid URL for the audio file
    pub audio_url: String,
    /// Title
    pub title: String,
    /// Caption, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the audio caption. See [`formatting options`](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Performer
    pub performer: Option<String>,
    /// Audio duration in seconds
    pub audio_duration: Option<i64>,
    /// [`Inline keyboard`](https://core.telegram.org/bots/features#inline-keyboards) attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the audio
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultAudio {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        audio_url: impl Into<String>,
        title: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            audio_url: audio_url.into(),
            title: title.into(),
            ..Default::default()
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
    pub fn audio_url(self, val: impl Into<String>) -> Self {
        Self {
            audio_url: val.into(),
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
    pub fn performer(self, val: impl Into<String>) -> Self {
        Self {
            performer: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn audio_duration(self, val: i64) -> Self {
        Self {
            audio_duration: Some(val),
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

impl Default for InlineQueryResultAudio {
    #[must_use]
    fn default() -> Self {
        Self {
            result_type: audio(),
            id: String::default(),
            audio_url: String::default(),
            title: String::default(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            performer: None,
            audio_duration: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

fn audio() -> String {
    InlineQueryResultType::Audio.into()
}
