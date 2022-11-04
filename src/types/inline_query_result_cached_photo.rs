use super::{InlineKeyboardMarkup, InputMessageContent, MessageEntity};

use serde::{Deserialize, Serialize};

/// Represents a link to a photo stored on the Telegram servers. By default, this photo will be sent by the user with an optional caption. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the photo.
/// <https://core.telegram.org/bots/api#inlinequeryresultcachedphoto>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultCachedPhoto {
    /// Type of the result, must be *photo*
    #[serde(rename = "type", default = "photo")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid file identifier of the photo
    pub photo_file_id: String,
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
    /// *Optional*. `Inline keyboard <https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating>` attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// *Optional*. Content of the message to be sent instead of the photo
    pub input_message_content: Option<InputMessageContent>,
}

impl Default for InlineQueryResultCachedPhoto {
    fn default() -> Self {
        Self {
            result_type: photo(),
            id: String::default(),
            photo_file_id: String::default(),
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

fn photo() -> String {
    "photo".to_string()
}
