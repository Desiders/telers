use super::{InputFile, MessageEntity};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Represents an audio file to be treated as music to be sent.
/// <https://core.telegram.org/bots/api#inputmediaaudio>
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct InputMediaAudio<'a> {
    /// Type of the result, must be *audio*
    #[serde(rename = "type")]
    pub media_type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass 'attach://<file_attach_name>' to upload a new one using multipart/form-data under <file_attach_name> name. :ref:`More information on Sending Files » <sending-files>`
    pub media: InputFile<'a>,
    /// *Optional*. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass 'attach://<file_attach_name>' if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. :ref:`More information on Sending Files » <sending-files>`
    pub thumb: Option<InputFile<'a>>,
    /// *Optional*. Caption of the audio to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// *Optional*. Mode for parsing entities in the audio caption. See `formatting options <https://core.telegram.org/bots/api#formatting-options>` for more details.
    pub parse_mode: Option<String>,
    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// *Optional*. Duration of the audio in seconds
    pub duration: Option<i64>,
    /// *Optional*. Performer of the audio
    pub performer: Option<String>,
    /// *Optional*. Title of the audio
    pub title: Option<String>,
}

impl<'a> InputMediaAudio<'a> {
    #[must_use]
    pub fn new(media: InputFile<'a>) -> Self {
        Self {
            media_type: "audio".to_string(),
            media,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            duration: None,
            performer: None,
            title: None,
        }
    }

    #[must_use]
    pub fn media(mut self, val: InputFile<'a>) -> Self {
        self.media = val;
        self
    }

    #[must_use]
    pub fn thumb(mut self, val: InputFile<'a>) -> Self {
        self.thumb = Some(val);
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
    pub fn duration(mut self, val: i64) -> Self {
        self.duration = Some(val);
        self
    }

    #[must_use]
    pub fn performer<T: Into<String>>(mut self, val: T) -> Self {
        self.performer = Some(val.into());
        self
    }

    #[must_use]
    pub fn title<T: Into<String>>(mut self, val: T) -> Self {
        self.title = Some(val.into());
        self
    }
}
