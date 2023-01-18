use super::{InlineKeyboardMarkup, InputMessageContent, MessageEntity};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the animation.
/// <https://core.telegram.org/bots/api#inlinequeryresultmpeg4gif>
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultMpeg4Gif {
    /// Type of the result, must be *mpeg4_gif*
    #[serde(rename = "type")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid URL for the MPEG4 file. File size must not exceed 1MB
    pub mpeg4_url: String,
    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    pub thumb_url: String,
    /// *Optional*. Video width
    pub mpeg4_width: Option<i64>,
    /// *Optional*. Video height
    pub mpeg4_height: Option<i64>,
    /// Optional*. Video duration in seconds
    pub mpeg4_duration: Option<i64>,
    /// *Optional*. MIME type of the thumbnail, must be one of 'image/jpeg', 'image/gif', or 'video/mp4'. Defaults to 'image/jpeg'
    pub thumb_mime_type: Option<String>,
    /// *Optional*. Title for the result
    pub title: Option<String>,
    /// *Optional*. Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// *Optional*. Mode for parsing entities in the caption. See `formatting options <https://core.telegram.org/bots/api#formatting-options>` for more details.
    pub parse_mode: Option<String>,
    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// *Optional*. `Inline keyboard <https://core.telegram.org/bots/features#inline-keyboards>` attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// *Optional*. Content of the message to be sent instead of the video animation
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultMpeg4Gif {
    #[must_use]
    pub fn new<T: Into<String>>(id: T, mpeg4_url: T, thumb_url: T) -> Self {
        Self {
            id: id.into(),
            mpeg4_url: mpeg4_url.into(),
            thumb_url: thumb_url.into(),
            ..Default::default()
        }
    }

    pub fn id<T: Into<String>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    pub fn mpeg4_url<T: Into<String>>(mut self, val: T) -> Self {
        self.mpeg4_url = val.into();
        self
    }

    pub fn thumb_url<T: Into<String>>(mut self, val: T) -> Self {
        self.thumb_url = val.into();
        self
    }

    pub fn mpeg4_width(mut self, val: i64) -> Self {
        self.mpeg4_width = Some(val);
        self
    }

    pub fn mpeg4_height(mut self, val: i64) -> Self {
        self.mpeg4_height = Some(val);
        self
    }

    pub fn mpeg4_duration(mut self, val: i64) -> Self {
        self.mpeg4_duration = Some(val);
        self
    }

    pub fn thumb_mime_type<T: Into<String>>(mut self, val: T) -> Self {
        self.thumb_mime_type = Some(val.into());
        self
    }

    pub fn title<T: Into<String>>(mut self, val: T) -> Self {
        self.title = Some(val.into());
        self
    }

    pub fn caption<T: Into<String>>(mut self, val: T) -> Self {
        self.caption = Some(val.into());
        self
    }

    pub fn parse_mode<T: Into<String>>(mut self, val: T) -> Self {
        self.parse_mode = Some(val.into());
        self
    }

    pub fn caption_entities(mut self, val: Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(val);
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }

    pub fn input_message_content(mut self, val: InputMessageContent) -> Self {
        self.input_message_content = Some(val);
        self
    }
}

impl Default for InlineQueryResultMpeg4Gif {
    fn default() -> Self {
        Self {
            result_type: "mpeg4_gif".to_string(),
            id: String::default(),
            mpeg4_url: String::default(),
            thumb_url: String::default(),
            mpeg4_width: None,
            mpeg4_height: None,
            mpeg4_duration: None,
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
