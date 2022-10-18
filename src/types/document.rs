use super::PhotoSize;

use serde::{Deserialize, Serialize};

/// This object represents a general file (as opposed to `photos <https://core.telegram.org/bots/api#photosize>`_, `voice messages <https://core.telegram.org/bots/api#voice>`_ and `audio files <https://core.telegram.org/bots/api#audio>`_).
/// <https://core.telegram.org/bots/api#document>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Document {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// *Optional*. Document thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// *Optional*. Original filename as defined by sender
    pub file_name: Option<String>,
    /// *Optional*. MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// *Optional*. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    pub file_size: Option<i64>,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            file_id: String::default(),
            file_unique_id: String::default(),
            thumb: None,
            file_name: None,
            mime_type: None,
            file_size: None,
        }
    }
}
