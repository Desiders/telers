/// This object represents a service message about a video chat started in the chat. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#videochatstarted>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
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
