use super::PhotoSize;

use serde::Deserialize;

/// This object represents a general file (as opposed to [photos](crate::types::PhotoSize), [voice messages](crate::types::Voice) and [audio files](crate::types::Audio).
/// # Documentation
/// <https://core.telegram.org/bots/api#document>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
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
