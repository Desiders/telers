use super::PhotoSize;

use serde::Deserialize;

/// This object represent a user's profile pictures.
/// # Documentation
/// <https://core.telegram.org/bots/api#userprofilephotos>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has
    pub total_count: i64,
    /// Requested profile pictures (in up to 4 sizes each)
    pub photos: Box<[Box<[PhotoSize]>]>,
}
