use super::{InlineKeyboardMarkup, InputMessageContent, MessageEntity};

use crate::enums::InlineQueryResultType;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a link to an animated GIF file. By default, this animated GIF file will be sent by the user with optional caption. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the animation.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultgif>
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultGif {
    /// Type of the result, must be *gif*
    #[serde(rename = "type", default = "gif")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid URL for the GIF file. File size must not exceed 1MB
    pub gif_url: String,
    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    pub thumb_url: String,
    /// Width of the GIF
    pub gif_width: Option<i64>,
    /// Height of the GIF
    pub gif_height: Option<i64>,
    /// Duration of the GIF in seconds
    pub gif_duration: Option<i64>,
    /// MIME type of the thumbnail, must be one of 'image/jpeg', 'image/gif', or 'video/mp4'. Defaults to 'image/jpeg'
    pub thumb_mime_type: Option<String>,
    /// Title for the result
    pub title: Option<String>,
    /// Caption of the GIF file to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the caption. See [`formatting options`](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// [`Inline keyboard`](https://core.telegram.org/bots/features#inline-keyboards) attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the GIF animation
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultGif {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        gif_url: impl Into<String>,
        thumb_url: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            gif_url: gif_url.into(),
            thumb_url: thumb_url.into(),
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
    pub fn gif_url(self, val: impl Into<String>) -> Self {
        Self {
            gif_url: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn thumb_url(self, val: impl Into<String>) -> Self {
        Self {
            thumb_url: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn gif_width(self, val: i64) -> Self {
        Self {
            gif_width: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn gif_height(self, val: i64) -> Self {
        Self {
            gif_height: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn gif_duration(self, val: i64) -> Self {
        Self {
            gif_duration: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn thumb_mime_type(self, val: impl Into<String>) -> Self {
        Self {
            thumb_mime_type: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn title(self, val: impl Into<String>) -> Self {
        Self {
            title: Some(val.into()),
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

impl Default for InlineQueryResultGif {
    #[must_use]
    fn default() -> Self {
        Self {
            result_type: gif(),
            id: String::default(),
            gif_url: String::default(),
            thumb_url: String::default(),
            gif_width: None,
            gif_height: None,
            gif_duration: None,
            thumb_mime_type: None,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

fn gif() -> String {
    InlineQueryResultType::Gif.into()
}
