use serde::{Deserialize, Serialize};
/// Media is a live photo, information about the live photo
/// # Notes
/// This object represents a poll media from original field `live_photo`.
/// # Documentation
/// <https://core.telegram.org/bots/api#pollmedia>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PollMediaLivePhoto {
    /// Media is a live photo, information about the live photo
    pub live_photo: crate::types::LivePhoto,
}
impl PollMediaLivePhoto {
    /// Creates a new `PollMediaLivePhoto`.
    ///
    /// # Arguments
    /// * `live_photo` - Media is a live photo, information about the live photo
    #[must_use]
    pub fn new<T0: Into<crate::types::LivePhoto>>(live_photo: T0) -> Self {
        Self {
            live_photo: live_photo.into(),
        }
    }

    /// Media is a live photo, information about the live photo
    #[must_use]
    pub fn live_photo<T: Into<crate::types::LivePhoto>>(mut self, val: T) -> Self {
        self.live_photo = val.into();
        self
    }
}
