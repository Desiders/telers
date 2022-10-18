use serde::{Deserialize, Serialize};

/// This object represents a service message about a video chat started in the chat. Currently holds no information.
/// <https://core.telegram.org/bots/api#videochatstarted>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct VideoChatStarted {}
