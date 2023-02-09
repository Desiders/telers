use super::{InlineKeyboardMarkup, InputMessageContent, MessageEntity};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a link to a photo. By default, this photo will be sent by the user with optional caption. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the photo.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultphoto>
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultPhoto {
    /// Type of the result, must be *photo*
    #[serde(rename = "type")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid URL of the photo. Photo must be in **JPEG** format. Photo size must not exceed 5MB
    pub photo_url: String,
    /// URL of the thumbnail for the photo
    pub thumb_url: String,
    /// *Optional*. Width of the photo
    pub photo_width: Option<i64>,
    /// *Optional*. Height of the photo
    pub photo_height: Option<i64>,
    /// *Optional*. Title for the result
    pub title: Option<String>,
    /// *Optional*. Short description of the result
    pub description: Option<String>,
    /// *Optional*. Caption of the photo to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// *Optional*. Mode for parsing entities in the photo caption. See `formatting options <https://core.telegram.org/bots/api#formatting-options>` for more details.
    pub parse_mode: Option<String>,
    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// *Optional*. `Inline keyboard <https://core.telegram.org/bots/features#inline-keyboards>` attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// *Optional*. Content of the message to be sent instead of the photo
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultPhoto {
    #[must_use]
    pub fn new<T: Into<String>>(id: T, photo_url: T, thumb_url: T) -> Self {
        Self {
            id: id.into(),
            photo_url: photo_url.into(),
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
    pub fn photo_url<T: Into<String>>(mut self, val: T) -> Self {
        self.photo_url = val.into();
        self
    }

    #[must_use]
    pub fn thumb_url<T: Into<String>>(mut self, val: T) -> Self {
        self.thumb_url = val.into();
        self
    }

    #[must_use]
    pub fn photo_width(mut self, val: i64) -> Self {
        self.photo_width = Some(val);
        self
    }

    #[must_use]
    pub fn photo_height(mut self, val: i64) -> Self {
        self.photo_height = Some(val);
        self
    }

    #[must_use]
    pub fn title<T: Into<String>>(mut self, val: T) -> Self {
        self.title = Some(val.into());
        self
    }

    #[must_use]
    pub fn description<T: Into<String>>(mut self, val: T) -> Self {
        self.description = Some(val.into());
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

impl Default for InlineQueryResultPhoto {
    #[must_use]
    fn default() -> Self {
        Self {
            result_type: "photo".to_string(),
            id: String::default(),
            photo_url: String::default(),
            thumb_url: String::default(),
            photo_width: None,
            photo_height: None,
            title: None,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}
