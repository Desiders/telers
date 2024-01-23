use serde::Deserialize;

/// This object represents a service message about a video chat started in the chat. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#videochatstarted>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct VideoChatStarted {}
