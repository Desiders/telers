use serde::{Deserialize, Serialize};
/// This object represents a service message about a video chat ended in the chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#videochatended>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideoChatEnded {
    /// Video chat duration in seconds
    pub duration: i64,
}
impl VideoChatEnded {
    /// Creates a new `VideoChatEnded`.
    ///
    /// # Arguments
    /// * `duration` - Video chat duration in seconds
    #[must_use]
    pub fn new<T0: Into<i64>>(duration: T0) -> Self {
        Self {
            duration: duration.into(),
        }
    }

    /// Video chat duration in seconds
    #[must_use]
    pub fn duration<T: Into<i64>>(mut self, val: T) -> Self {
        self.duration = val.into();
        self
    }
}
