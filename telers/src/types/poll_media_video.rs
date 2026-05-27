use serde::{Deserialize, Serialize};
/// Media is a video, information about the video
/// # Notes
/// This object represents a poll media from original field `video`.
/// # Documentation
/// <https://core.telegram.org/bots/api#pollmedia>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PollMediaVideo {
    /// Media is a video, information about the video
    pub video: Box<crate::types::Video>,
}
impl PollMediaVideo {
    /// Creates a new `PollMediaVideo`.
    ///
    /// # Arguments
    /// * `video` - Media is a video, information about the video
    #[must_use]
    pub fn new<T0: Into<crate::types::Video>>(video: T0) -> Self {
        Self {
            video: Box::new(video.into()),
        }
    }

    /// Media is a video, information about the video
    #[must_use]
    pub fn video<T: Into<crate::types::Video>>(mut self, val: T) -> Self {
        self.video = Box::new(val.into());
        self
    }
}
