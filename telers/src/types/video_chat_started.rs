use serde::{Deserialize, Serialize};
/// This object represents a service message about a video chat started in the chat. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#videochatstarted>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideoChatStarted {}
impl VideoChatStarted {
    /// Creates a new `VideoChatStarted`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for VideoChatStarted {
    fn default() -> Self {
        Self::new()
    }
}
