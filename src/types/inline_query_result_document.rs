use super::{InlineKeyboardMarkup, InputMessageContent, MessageEntity};

use serde::{Deserialize, Serialize};

/// Represents a link to a file. By default, this file will be sent by the user with an optional caption. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the file. Currently, only **.PDF** and **.ZIP** files can be sent using this method.
/// **Note:** This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
/// <https://core.telegram.org/bots/api#inlinequeryresultdocument>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultDocument {
    /// Type of the result, must be *document*
    #[serde(rename = "type", default = "document")]
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

impl Default for InlineQueryResultDocument {
    fn default() -> Self {
        Self {
            result_type: document(),
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

fn document() -> String {
    "document".to_string()
}
