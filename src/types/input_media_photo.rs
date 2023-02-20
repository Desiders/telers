use super::{InputFile, MessageEntity};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Represents a photo to be sent.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputmediaphoto>
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct InputMediaPhoto<'a> {
    /// Type of the result, must be *photo*
    #[serde(rename = "type")]
    pub media_type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass 'attach://<file_attach_name>' to upload a new one using `multipart/form-data` under <file_attach_name> name. [`More information on Sending Files`](https://core.telegram.org/bots/api#sending-files).
    pub media: InputFile<'a>,
    /// *Optional*. Caption of the photo to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// *Optional*. Mode for parsing entities in the photo caption. See [`formatting options`](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// *Optional*. Pass `True` if the photo needs to be covered with a spoiler animation
    pub has_spoiler: Option<bool>,
}

impl<'a> InputMediaPhoto<'a> {
    #[must_use]
    pub fn new<T: Into<InputFile<'a>>>(media: T) -> Self {
        Self {
            media_type: "photo".to_string(),
            media: media.into(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            has_spoiler: None,
        }
    }

    #[must_use]
    pub fn media<T: Into<InputFile<'a>>>(mut self, val: T) -> Self {
        self.media = val.into();
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
    pub fn has_spoiler(mut self, val: bool) -> Self {
        self.has_spoiler = Some(val);
        self
    }
}
