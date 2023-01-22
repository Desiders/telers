use super::{InputFile, MessageEntity};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Represents a general file to be sent.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputmediadocument>
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct InputMediaDocument<'a> {
    /// Type of the result, must be *document*
    #[serde(rename = "type")]
    pub media_type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass 'attach://<file_attach_name>' to upload a new one using `multipart/form-data` under <file_attach_name> name. `More information on Sending Files <https://core.telegram.org/bots/api#sending-files>`.
    pub media: InputFile<'a>,
    /// *Optional*. Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using `multipart/form-data`. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass 'attach://<file_attach_name>' if the thumbnail was uploaded using `multipart/form-data` under <file_attach_name>. `More information on Sending Files <https://core.telegram.org/bots/api#sending-files>`.
    pub thumb: Option<InputFile<'a>>,
    /// *Optional*. Caption of the document to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// *Optional*. Mode for parsing entities in the document caption. See `formatting options <https://core.telegram.org/bots/api#formatting-options>` for more details.
    pub parse_mode: Option<String>,
    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// *Optional*. Disables automatic server-side content type detection for files uploaded using `multipart/form-data`. Always `True`, if the document is sent as part of an album.
    pub disable_content_type_detection: Option<bool>,
}

impl<'a> InputMediaDocument<'a> {
    #[must_use]
    pub fn new<T: Into<InputFile<'a>>>(media: T) -> Self {
        Self {
            media_type: "document".to_string(),
            media: media.into(),
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_content_type_detection: None,
        }
    }

    #[must_use]
    pub fn media<T: Into<InputFile<'a>>>(mut self, val: T) -> Self {
        self.media = val.into();
        self
    }

    #[must_use]
    pub fn thumb<T: Into<InputFile<'a>>>(mut self, val: T) -> Self {
        self.thumb = Some(val.into());
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
    pub fn disable_content_type_detection(mut self, val: bool) -> Self {
        self.disable_content_type_detection = Some(val);
        self
    }
}
