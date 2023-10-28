use super::{InlineKeyboardMarkup, InputMessageContent, MessageEntity};

use crate::enums::InlineQueryResultType;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the animation.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultmpeg4gif>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultMpeg4Gif {
    /// Type of the result, must be *mpeg4_gif*
    #[serde(rename = "type", default = "mpeg4_gif")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid URL for the MPEG4 file. File size must not exceed 1MB
    pub mpeg4_url: String,
    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    pub thumbnail_url: String,
    /// Video width
    pub mpeg4_width: Option<i64>,
    /// Video height
    pub mpeg4_height: Option<i64>,
    /// Optional*. Video duration in seconds
    pub mpeg4_duration: Option<i64>,
    /// MIME type of the thumbnail, must be one of 'image/jpeg', 'image/gif', or 'video/mp4'. Defaults to 'image/jpeg'
    pub thumbnail_mime_type: Option<String>,
    /// Title for the result
    pub title: Option<String>,
    /// Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the caption. See [`formatting options`](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// [`Inline keyboard`](https://core.telegram.org/bots/features#inline-keyboards) attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the video animation
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultMpeg4Gif {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        mpeg4_url: impl Into<String>,
        thumbnail_url: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            mpeg4_url: mpeg4_url.into(),
            thumbnail_url: thumbnail_url.into(),
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
    pub fn mpeg4_url(self, val: impl Into<String>) -> Self {
        Self {
            mpeg4_url: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn thumbnail_url(self, val: impl Into<String>) -> Self {
        Self {
            thumbnail_url: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn mpeg4_width(self, val: i64) -> Self {
        Self {
            mpeg4_width: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn mpeg4_height(self, val: i64) -> Self {
        Self {
            mpeg4_height: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn mpeg4_duration(self, val: i64) -> Self {
        Self {
            mpeg4_duration: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn thumbnail_mime_type(self, val: impl Into<String>) -> Self {
        Self {
            thumbnail_mime_type: Some(val.into()),
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

impl Default for InlineQueryResultMpeg4Gif {
    #[must_use]
    fn default() -> Self {
        Self {
            result_type: mpeg4_gif(),
            id: String::default(),
            mpeg4_url: String::default(),
            thumbnail_url: String::default(),
            mpeg4_width: None,
            mpeg4_height: None,
            mpeg4_duration: None,
            thumbnail_mime_type: None,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

fn mpeg4_gif() -> String {
    InlineQueryResultType::Mpeg4Gif.into()
}
