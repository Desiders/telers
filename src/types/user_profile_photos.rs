use super::PhotoSize;

use serde::{Deserialize, Serialize};

/// This object represent a user's profile pictures.
/// <https://core.telegram.org/bots/api#userprofilephotos>_
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has
    pub total_count: i64,
    /// Requested profile pictures (in up to 4 sizes each)
    pub photos: Vec<Vec<PhotoSize>>,
}
