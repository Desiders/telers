use super::{InputFile, MessageEntity};

use crate::enums::InputMediaType;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Represents an audio file to be treated as music to be sent.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputmediaaudio>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Serialize)]
pub struct InputMediaAudio<'a> {
    /// Type of the result, must be *audio*
    #[serde(rename = "type", default = "audio")]
    pub media_type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass 'attach://<file_attach_name>' to upload a new one using `multipart/form-data` under <file_attach_name> name. [`More information on Sending Files`](https://core.telegram.org/bots/api#sending-files).
    pub media: InputFile<'a>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using `multipart/form-data`. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass 'attach://<file_attach_name>' if the thumbnail was uploaded using `multipart/form-data` under <file_attach_name>. [`More information on Sending Files`](https://core.telegram.org/bots/api#sending-files).
    pub thumbnail: Option<InputFile<'a>>,
    /// Caption of the audio to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the audio caption. See [`formatting options`](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Duration of the audio in seconds
    pub duration: Option<i64>,
    /// Performer of the audio
    pub performer: Option<String>,
    /// Title of the audio
    pub title: Option<String>,
}

impl<'a> InputMediaAudio<'a> {
    #[must_use]
    pub fn new<T: Into<InputFile<'a>>>(media: T) -> Self {
        Self {
            media_type: audio(),
            media: media.into(),
            thumbnail: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            duration: None,
            performer: None,
            title: None,
        }
    }

    #[must_use]
    pub fn media(self, val: impl Into<InputFile<'a>>) -> Self {
        Self {
            media: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn thumb(self, val: impl Into<InputFile<'a>>) -> Self {
        Self {
            thumbnail: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn caption(self, val: impl Into<String>) -> Self {
        Self {
            caption: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn parse_mode(self, val: impl Into<String>) -> Self {
        Self {
            parse_mode: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn caption_entity(self, val: MessageEntity) -> Self {
        Self {
            caption_entities: Some(
                self.caption_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(Some(val))
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn caption_entities(self, val: impl IntoIterator<Item = MessageEntity>) -> Self {
        Self {
            caption_entities: Some(
                self.caption_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn duration(self, val: i64) -> Self {
        Self {
            duration: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn performer(self, val: impl Into<String>) -> Self {
        Self {
            performer: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn title(self, val: impl Into<String>) -> Self {
        Self {
            title: Some(val.into()),
            ..self
        }
    }
}

fn audio() -> String {
    InputMediaType::Audio.into()
}
