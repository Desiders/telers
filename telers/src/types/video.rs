use super::PhotoSize;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object represents a video file.
/// # Documentation
/// <https://core.telegram.org/bots/api#video>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct Video {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: Box<str>,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: Box<str>,
    /// Video width as defined by sender
    pub width: i64,
    /// Video height as defined by sender
    pub height: i64,
    /// Duration of the video in seconds as defined by sender
    pub duration: i64,
    /// Video thumbnail
    pub thumbnail: Option<PhotoSize>,
    /// Available sizes of the cover of the video in the message
    pub cover: Option<Box<[PhotoSize]>>,
    /// Timestamp in seconds from which the video will play in the message
    pub start_timestamp: Option<i64>,
    /// Original filename as defined by sender
    pub file_name: Option<Box<str>>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<Box<str>>,
    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    pub file_size: Option<i64>,
}
