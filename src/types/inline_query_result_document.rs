use super::{InlineKeyboardMarkup, InputMessageContent, MessageEntity};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a link to a file. By default, this file will be sent by the user with an optional caption. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the file. Currently, only **.PDF** and **.ZIP** files can be sent using this method.
/// **Note:** This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
/// <https://core.telegram.org/bots/api#inlinequeryresultdocument>
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultDocument {
    /// Type of the result, must be *document*
    #[serde(rename = "type")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A Title for the result
    pub title: String,
    /// A valid URL for the file
    pub document_url: String,
    /// MIME type of the content of the file, either 'application/pdf' or 'application/zip'
    pub mime_type: String,
    /// *Optional*. Caption of the document to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// *Optional*. Mode for parsing entities in the document caption. See `formatting options <https://core.telegram.org/bots/api#formatting-options>` for more details.
    pub parse_mode: Option<String>,
    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// *Optional*. Short description of the result
    pub description: Option<String>,
    /// *Optional*. Inline keyboard attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// *Optional*. Content of the message to be sent instead of the file
    pub input_message_content: Option<InputMessageContent>,
    /// *Optional*. URL of the thumbnail (JPEG only) for the file
    pub thumb_url: Option<String>,
    /// *Optional*. Thumbnail width
    pub thumb_width: Option<i64>,
    /// *Optional*. Thumbnail height
    pub thumb_height: Option<i64>,
}

impl InlineQueryResultDocument {
    #[must_use]
    pub fn new<T: Into<String>>(
        id: T,
        title: T,
        document_url: T,
        mime_type: T,
    ) -> InlineQueryResultDocument {
        Self {
            id: id.into(),
            title: title.into(),
            document_url: document_url.into(),
            mime_type: mime_type.into(),
            ..Default::default()
        }
    }

    pub fn id<T: Into<String>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    pub fn title<T: Into<String>>(mut self, val: T) -> Self {
        self.title = val.into();
        self
    }

    pub fn document_url<T: Into<String>>(mut self, val: T) -> Self {
        self.document_url = val.into();
        self
    }

    pub fn mime_type<T: Into<String>>(mut self, val: T) -> Self {
        self.mime_type = val.into();
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

    pub fn description<T: Into<String>>(mut self, val: T) -> Self {
        self.description = Some(val.into());
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

    pub fn thumb_url<T: Into<String>>(mut self, val: T) -> Self {
        self.thumb_url = Some(val.into());
        self
    }

    pub fn thumb_width(mut self, val: i64) -> Self {
        self.thumb_width = Some(val);
        self
    }

    pub fn thumb_height(mut self, val: i64) -> Self {
        self.thumb_height = Some(val);
        self
    }
}

impl Default for InlineQueryResultDocument {
    fn default() -> Self {
        Self {
            result_type: "document".to_string(),
            id: String::default(),
            title: String::default(),
            document_url: String::default(),
            mime_type: String::default(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            description: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}
