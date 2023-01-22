use serde::Deserialize;

/// This object represents a service message about a video chat started in the chat. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#videochatstarted>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct VideoChatStarted {}
