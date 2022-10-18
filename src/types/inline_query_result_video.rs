use super::{InlineKeyboardMarkup, InputMessageContent, MessageEntity};

use serde::{Deserialize, Serialize};

/// Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the video.
/// If an `InlineQueryResultVideo` message contains an embedded video (e.g., `YouTube`), you **must** replace its content using `input_message_content`.
/// <https://core.telegram.org/bots/api#inlinequeryresultvideo>_
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultVideo {
    /// Type of the result, must be *video*
    #[serde(rename = "type", default = "video")]
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
    /// *Optional*. Mode for parsing entities in the video caption. See `formatting options <https://core.telegram.org/bots/api#formatting-options>`_ for more details.
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
    /// *Optional*. `Inline keyboard <https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating>`_ attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// *Optional*. Content of the message to be sent instead of the video. This field is **required** if InlineQueryResultVideo is used to send an HTML-page as a result (e.g., a YouTube video).
    pub input_message_content: Option<InputMessageContent>,
}

impl Default for InlineQueryResultVideo {
    fn default() -> Self {
        Self {
            result_type: video(),
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

fn video() -> String {
    "video".to_string()
}
