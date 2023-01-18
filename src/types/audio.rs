use super::PhotoSize;

use serde::Deserialize;

/// This object represents an audio file to be treated as music by the Telegram clients.
/// <https://core.telegram.org/bots/api#audio>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct Audio {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Duration of the audio in seconds as defined by sender
    pub duration: i64,
    /// *Optional*. Performer of the audio as defined by sender or by audio tags
    pub performer: Option<String>,
    /// *Optional*. Title of the audio as defined by sender or by audio tags
    pub title: Option<String>,
    /// *Optional*. Original filename as defined by sender
    pub file_name: Option<String>,
    /// *Optional*. MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// *Optional*. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    pub file_size: Option<i64>,
    /// *Optional*. Thumbnail of the album cover to which the music file belongs
    pub thumb: Option<PhotoSize>,
}
