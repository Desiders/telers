use super::{InlineKeyboardMarkup, InputMessageContent, MessageEntity};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the video.
/// If an `InlineQueryResultVideo` message contains an embedded video (e.g., `YouTube`), you **must** replace its content using `input_message_content`.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultvideo>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultVideo {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid URL for the embedded video player or video file
    pub video_url: String,
    /// MIME type of the content of the video URL, 'text/html' or 'video/mp4'
    pub mime_type: String,
    /// URL of the thumbnail (JPEG only) for the video
    pub thumbnail_url: String,
    /// Title for the result
    pub title: String,
    /// Caption of the video to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the video caption. See [`formatting options`](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Video width
    pub video_width: Option<i64>,
    /// Video height
    pub video_height: Option<i64>,
    /// Video duration in seconds
    pub video_duration: Option<i64>,
    /// Short description of the result
    pub description: Option<String>,
    /// [`Inline keyboard`](https://core.telegram.org/bots/features#inline-keyboards) attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the video. This field is **required** if InlineQueryResultVideo is used to send an HTML-page as a result (e.g., a YouTube video).
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultVideo {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        video_url: impl Into<String>,
        mime_type: impl Into<String>,
        thumbnail_url: impl Into<String>,
        title: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            video_url: video_url.into(),
            mime_type: mime_type.into(),
            thumbnail_url: thumbnail_url.into(),
            title: title.into(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            video_width: None,
            video_height: None,
            video_duration: None,
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
    pub fn video_url(self, val: impl Into<String>) -> Self {
        Self {
            video_url: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn mime_type(self, val: impl Into<String>) -> Self {
        Self {
            mime_type: val.into(),
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
    pub fn video_width(self, val: i64) -> Self {
        Self {
            video_width: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn video_height(self, val: i64) -> Self {
        Self {
            video_height: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn video_duration(self, val: i64) -> Self {
        Self {
            video_duration: Some(val),
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

impl InlineQueryResultVideo {
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
    pub fn caption_entity_option(self, val: Option<MessageEntity>) -> Self {
        Self {
            caption_entities: val.map(|v| vec![v]),
            ..self
        }
    }

    #[must_use]
    pub fn caption_entities_option(self, val: Option<Vec<MessageEntity>>) -> Self {
        Self {
            caption_entities: val,
            ..self
        }
    }

    #[must_use]
    pub fn video_width_option(self, val: Option<i64>) -> Self {
        Self {
            video_width: val,
            ..self
        }
    }

    #[must_use]
    pub fn video_height_option(self, val: Option<i64>) -> Self {
        Self {
            video_height: val,
            ..self
        }
    }

    #[must_use]
    pub fn video_duration_option(self, val: Option<i64>) -> Self {
        Self {
            video_duration: val,
            ..self
        }
    }

    #[must_use]
    pub fn description_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            description: val.map(Into::into),
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

    #[must_use]
    pub fn input_message_content_option(self, val: Option<impl Into<InputMessageContent>>) -> Self {
        Self {
            input_message_content: val.map(Into::into),
            ..self
        }
    }
}
