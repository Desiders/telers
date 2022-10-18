use super::{InputMediaKind, MessageEntity};

use serde::{Deserialize, Serialize};

/// Represents a photo to be sent.
/// <https://core.telegram.org/bots/api#inputmediaphoto>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InputMediaPhoto {
    /// Type of the result, must be *photo*
    #[serde(rename = "type", default = "photo")]
    pub media_type: String,
    /// File to send. Pass a file_id to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass 'attach://<file_attach_name>' to upload a new one using multipart/form-data under <file_attach_name> name. :ref:`More information on Sending Files » <sending-files>`
    pub media: InputMediaKind,
    /// *Optional*. Caption of the photo to be sent, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// *Optional*. Mode for parsing entities in the photo caption. See `formatting options <https://core.telegram.org/bots/api#formatting-options>`_ for more details.
    pub parse_mode: Option<String>,
    /// *Optional*. List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
}

impl Default for InputMediaPhoto {
    fn default() -> Self {
        Self {
            media_type: photo(),
            media: InputMediaKind::FileIdOrUrl(String::default()),
            caption: None,
            parse_mode: None,
            caption_entities: None,
        }
    }
}

fn photo() -> String {
    "photo".to_string()
}
