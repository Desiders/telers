use super::{InlineKeyboardMarkup, InputMessageContent, MessageEntity};

use serde::{Deserialize, Serialize};

/// Represents a link to a video file stored on the Telegram servers. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the video.
/// <https://core.telegram.org/bots/api#inlinequeryresultcachedvideo>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultCachedVideo {
    /// Type of the result, must be *video*
    #[serde(rename = "type", default = "video")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Title for the result
    pub title: String,
    /// A valid file identifier for the video file
    pub video_file_id: String,
    /// *Optional*. Caption of the video to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// *Optional*. Mode for parsing entities in the video caption. See `formatting options <https://core.telegram.org/bots/api#formatting-options>` for more details.
    pub parse_mode: Option<String>,
    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// *Optional*. Short description of the result
    pub description: Option<String>,
    /// *Optional*. `Inline keyboard <https://core.telegram.org/bots/features#inline-keyboards>` attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// *Optional*. Content of the message to be sent instead of the video. This field is **required** if InlineQueryResultVideo is used to send an HTML-page as a result (e.g., a YouTube video).
    pub input_message_content: Option<InputMessageContent>,
}

impl Default for InlineQueryResultCachedVideo {
    fn default() -> Self {
        Self {
            result_type: video(),
            id: String::default(),
            title: String::default(),
            video_file_id: String::default(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            description: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

fn video() -> String {
    "video".to_string()
}
