use super::{InputFile, MessageEntity};

use crate::enums::InputMediaType;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Represents an animation file (GIF or H.264/MPEG-4 AVC video without sound) to be sent.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputmediaanimation>
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct InputMediaAnimation<'a> {
    /// Type of the result, must be *animation*
    #[serde(rename = "type", default = "animation")]
    pub media_type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass 'attach://<file_attach_name>' to upload a new one using `multipart/form-data` under <file_attach_name> name. [`More information on Sending Files`](https://core.telegram.org/bots/api#sending-files).
    pub media: InputFile<'a>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using `multipart/form-data`. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass 'attach://<file_attach_name>' if the thumbnail was uploaded using `multipart/form-data` under <file_attach_name>. [`More information on Sending Files`](https://core.telegram.org/bots/api#sending-files).
    pub thumbnail: Option<InputFile<'a>>,
    /// Caption of the video to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Caption of the animation to be sent, 0-1024 characters after entities parsing"""
    pub parse_mode: Option<String>,
    /// Mode for parsing entities in the animation caption. See [`formatting options`](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Video width
    pub width: Option<i64>,
    /// Animation height
    pub height: Option<i64>,
    /// Animation duration in seconds
    pub duration: Option<i64>,
    /// Pass `True` if the animation needs to be covered with a spoiler animation
    pub has_spoiler: Option<bool>,
}

impl<'a> InputMediaAnimation<'a> {
    #[must_use]
    pub fn new<T: Into<InputFile<'a>>>(media: T) -> Self {
        Self {
            media_type: animation(),
            media: media.into(),
            thumbnail: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            width: None,
            height: None,
            duration: None,
            has_spoiler: None,
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
    pub fn width(self, val: i64) -> Self {
        Self {
            width: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn height(self, val: i64) -> Self {
        Self {
            height: Some(val),
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
    pub fn has_spoiler(self, val: bool) -> Self {
        Self {
            has_spoiler: Some(val),
            ..self
        }
    }
}

fn animation() -> String {
    InputMediaType::Animation.into()
}
