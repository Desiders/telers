use super::PhotoSize;

use serde::{Deserialize, Serialize};

/// This object represents a `video message <https://telegram.org/blog/video-messages-and-telescope>`_ (available in Telegram apps as of `v.4.0 <https://telegram.org/blog/video-messages-and-telescope>`_).
/// <https://core.telegram.org/bots/api#videonote>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct VideoNote {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Video width and height (diameter of the video message) as defined by sender
    pub length: i64,
    /// Duration of the video in seconds as defined by sender
    pub duration: i64,
    /// *Optional*. Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// *Optional*. File size in bytes
    pub file_size: Option<i64>,
}

impl Default for VideoNote {
    fn default() -> Self {
        Self {
            file_id: String::default(),
            file_unique_id: String::default(),
            length: 0,
            duration: 0,
            thumb: None,
            file_size: None,
        }
    }
}
