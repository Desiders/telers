use super::{InlineKeyboardMarkup, InputMessageContent, MessageEntity};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a link to an MP3 audio file stored on the Telegram servers. By default, this audio file will be sent by the user. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the audio.
/// **Note:** This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
/// <https://core.telegram.org/bots/api#inlinequeryresultcachedaudio>
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultCachedAudio {
    /// Type of the result, must be *audio*
    #[serde(rename = "type")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// A valid file identifier for the audio file
    pub audio_file_id: String,
    /// *Optional*. Caption, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// *Optional*. Mode for parsing entities in the audio caption. See `formatting options <https://core.telegram.org/bots/api#formatting-options>` for more details.
    pub parse_mode: Option<String>,
    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// *Optional*. `Inline keyboard <https://core.telegram.org/bots/features#inline-keyboards>` attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// *Optional*. Content of the message to be sent instead of the audio
    pub input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedAudio {
    #[must_use]
    pub fn new<T: Into<String>>(id: T, audio_file_id: T) -> Self {
        Self {
            id: id.into(),
            audio_file_id: audio_file_id.into(),
            ..Default::default()
        }
    }

    pub fn id<T: Into<String>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    pub fn audio_file_id<T: Into<String>>(mut self, val: T) -> Self {
        self.audio_file_id = val.into();
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

impl Default for InlineQueryResultCachedAudio {
    fn default() -> Self {
        Self {
            result_type: "audio".to_string(),
            id: String::default(),
            audio_file_id: String::default(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}
