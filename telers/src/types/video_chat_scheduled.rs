use serde::{Deserialize, Serialize};
/// This object represents a service message about a video chat scheduled in the chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#videochatscheduled>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideoChatScheduled {
    /// Point in time (Unix timestamp) when the video chat is supposed to be started by a chat administrator
    pub start_date: i64,
}
impl VideoChatScheduled {
    /// Creates a new `VideoChatScheduled`.
    ///
    /// # Arguments
    /// * `start_date` - Point in time (Unix timestamp) when the video chat is supposed to be started by a chat administrator
    #[must_use]
    pub fn new<T0: Into<i64>>(start_date: T0) -> Self {
        Self {
            start_date: start_date.into(),
        }
    }

    /// Point in time (Unix timestamp) when the video chat is supposed to be started by a chat administrator
    #[must_use]
    pub fn start_date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.start_date = val.into();
        this
    }
}
