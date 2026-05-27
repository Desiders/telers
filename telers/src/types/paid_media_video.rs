use serde::{Deserialize, Serialize};
/// The paid media is a video.
/// # Documentation
/// <https://core.telegram.org/bots/api#paidmediavideo>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaidMediaVideo {
    /// The video
    pub video: Box<crate::types::Video>,
}
impl PaidMediaVideo {
    /// Creates a new `PaidMediaVideo`.
    ///
    /// # Arguments
    /// * `video` - The video
    #[must_use]
    pub fn new<T0: Into<crate::types::Video>>(video: T0) -> Self {
        Self {
            video: Box::new(video.into()),
        }
    }

    /// The video
    #[must_use]
    pub fn video<T: Into<crate::types::Video>>(mut self, val: T) -> Self {
        self.video = Box::new(val.into());
        self
    }
}
