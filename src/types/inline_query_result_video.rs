use super::{InlineKeyboardMarkup, InputMessageContent, MessageEntity};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the video.
/// If an `InlineQueryResultVideo` message contains an embedded video (e.g., `YouTube`), you **must** replace its content using `input_message_content`.
/// <https://core.telegram.org/bots/api#inlinequeryresultvideo>
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultVideo {
    /// Type of the result, must be *video*
    #[serde(rename = "type")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid URL for the embedded video player or video file
    pub video_url: String,
    /// MIME type of the content of the video URL, 'text/html' or 'video/mp4'
    pub mime_type: String,
    /// URL of the thumbnail (JPEG only) for the video
    pub thumb_url: String,
    /// Title for the result
    pub title: String,
    /// *Optional*. Caption of the video to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// *Optional*. Mode for parsing entities in the video caption. See `formatting options <https://core.telegram.org/bots/api#formatting-options>` for more details.
    pub parse_mode: Option<String>,
    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// *Optional*. Video width
    pub video_width: Option<i64>,
    /// *Optional*. Video height
    pub video_height: Option<i64>,
    /// *Optional*. Video duration in seconds
    pub video_duration: Option<i64>,
    /// *Optional*. Short description of the result
    pub description: Option<String>,
    /// *Optional*. `Inline keyboard <https://core.telegram.org/bots/features#inline-keyboards>` attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// *Optional*. Content of the message to be sent instead of the video. This field is **required** if InlineQueryResultVideo is used to send an HTML-page as a result (e.g., a YouTube video).
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultVideo {
    #[must_use]
    pub fn new<T: Into<String>>(id: T, video_url: T, mime_type: T, thumb_url: T, title: T) -> Self {
        Self {
            id: id.into(),
            video_url: video_url.into(),
            mime_type: mime_type.into(),
            thumb_url: thumb_url.into(),
            title: title.into(),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn id<T: Into<String>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    #[must_use]
    pub fn video_url<T: Into<String>>(mut self, val: T) -> Self {
        self.video_url = val.into();
        self
    }

    #[must_use]
    pub fn mime_type<T: Into<String>>(mut self, val: T) -> Self {
        self.mime_type = val.into();
        self
    }

    #[must_use]
    pub fn thumb_url<T: Into<String>>(mut self, val: T) -> Self {
        self.thumb_url = val.into();
        self
    }

    #[must_use]
    pub fn title<T: Into<String>>(mut self, val: T) -> Self {
        self.title = val.into();
        self
    }

    #[must_use]
    pub fn caption<T: Into<String>>(mut self, val: T) -> Self {
        self.caption = Some(val.into());
        self
    }

    #[must_use]
    pub fn parse_mode<T: Into<String>>(mut self, val: T) -> Self {
        self.parse_mode = Some(val.into());
        self
    }

    #[must_use]
    pub fn caption_entities(mut self, val: Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(val);
        self
    }

    #[must_use]
    pub fn video_width(mut self, val: i64) -> Self {
        self.video_width = Some(val);
        self
    }

    #[must_use]
    pub fn video_height(mut self, val: i64) -> Self {
        self.video_height = Some(val);
        self
    }

    #[must_use]
    pub fn video_duration(mut self, val: i64) -> Self {
        self.video_duration = Some(val);
        self
    }

    #[must_use]
    pub fn description<T: Into<String>>(mut self, val: T) -> Self {
        self.description = Some(val.into());
        self
    }

    #[must_use]
    pub fn reply_markup<T: Into<InlineKeyboardMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }

    #[must_use]
    pub fn input_message_content(mut self, val: InputMessageContent) -> Self {
        self.input_message_content = Some(val);
        self
    }
}

impl Default for InlineQueryResultVideo {
    fn default() -> Self {
        Self {
            result_type: "video".to_string(),
            id: String::default(),
            video_url: String::default(),
            mime_type: String::default(),
            thumb_url: String::default(),
            title: String::default(),
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
}
