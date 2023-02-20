use super::{InlineKeyboardMarkup, InputMessageContent, MessageEntity};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a link to an animated GIF file. By default, this animated GIF file will be sent by the user with optional caption. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the animation.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultgif>
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultGif {
    /// Type of the result, must be *gif*
    #[serde(rename = "type")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid URL for the GIF file. File size must not exceed 1MB
    pub gif_url: String,
    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    pub thumb_url: String,
    /// *Optional*. Width of the GIF
    pub gif_width: Option<i64>,
    /// *Optional*. Height of the GIF
    pub gif_height: Option<i64>,
    /// *Optional*. Duration of the GIF in seconds
    pub gif_duration: Option<i64>,
    /// *Optional*. MIME type of the thumbnail, must be one of 'image/jpeg', 'image/gif', or 'video/mp4'. Defaults to 'image/jpeg'
    pub thumb_mime_type: Option<String>,
    /// *Optional*. Title for the result
    pub title: Option<String>,
    /// *Optional*. Caption of the GIF file to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// *Optional*. Mode for parsing entities in the caption. See [`formatting options`](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// *Optional*. [`Inline keyboard`](https://core.telegram.org/bots/features#inline-keyboards) attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// *Optional*. Content of the message to be sent instead of the GIF animation
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultGif {
    #[must_use]
    pub fn new<T: Into<String>>(id: T, gif_url: T, thumb_url: T) -> Self {
        Self {
            id: id.into(),
            gif_url: gif_url.into(),
            thumb_url: thumb_url.into(),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn id<T: Into<String>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    #[must_use]
    pub fn gif_url<T: Into<String>>(mut self, val: T) -> Self {
        self.gif_url = val.into();
        self
    }

    #[must_use]
    pub fn thumb_url<T: Into<String>>(mut self, val: T) -> Self {
        self.thumb_url = val.into();
        self
    }

    #[must_use]
    pub fn gif_width(mut self, val: i64) -> Self {
        self.gif_width = Some(val);
        self
    }

    #[must_use]
    pub fn gif_height(mut self, val: i64) -> Self {
        self.gif_height = Some(val);
        self
    }

    #[must_use]
    pub fn gif_duration(mut self, val: i64) -> Self {
        self.gif_duration = Some(val);
        self
    }

    #[must_use]
    pub fn thumb_mime_type<T: Into<String>>(mut self, val: T) -> Self {
        self.thumb_mime_type = Some(val.into());
        self
    }

    #[must_use]
    pub fn title<T: Into<String>>(mut self, val: T) -> Self {
        self.title = Some(val.into());
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

impl Default for InlineQueryResultGif {
    #[must_use]
    fn default() -> Self {
        Self {
            result_type: "gif".to_string(),
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
