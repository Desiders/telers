use super::{InlineKeyboardMarkup, InputMessageContent, MessageEntity};

use serde::{Deserialize, Serialize};

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers. By default, this animated MPEG-4 file will be sent by the user with an optional caption. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the animation.
/// <https://core.telegram.org/bots/api#inlinequeryresultcachedmpeg4gif>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultCachedMpeg4Gif {
    /// Type of the result, must be *mpeg4_gif*
    #[serde(rename = "type", default = "mpeg4_gif")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid file identifier for the MPEG4 file
    pub mpeg4_file_id: String,
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

impl Default for InlineQueryResultCachedMpeg4Gif {
    fn default() -> Self {
        Self {
            result_type: mpeg4_gif(),
            id: String::default(),
            mpeg4_file_id: String::default(),
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}

fn mpeg4_gif() -> String {
    "mpeg4_gif".to_string()
}
