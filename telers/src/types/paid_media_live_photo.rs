use serde::{Deserialize, Serialize};
/// The paid media is a live photo.
/// # Documentation
/// <https://core.telegram.org/bots/api#paidmedialivephoto>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaidMediaLivePhoto {
    /// The photo
    pub live_photo: crate::types::LivePhoto,
}
impl PaidMediaLivePhoto {
    /// Creates a new `PaidMediaLivePhoto`.
    ///
    /// # Arguments
    /// * `live_photo` - The photo
    #[must_use]
    pub fn new<T0: Into<crate::types::LivePhoto>>(live_photo: T0) -> Self {
        Self {
            live_photo: live_photo.into(),
        }
    }

    /// The photo
    #[must_use]
    pub fn live_photo<T: Into<crate::types::LivePhoto>>(mut self, val: T) -> Self {
        self.live_photo = val.into();
        self
    }
}
