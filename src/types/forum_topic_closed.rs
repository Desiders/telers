use serde::{Deserialize, Serialize};

/// This object represents a service message about a forum topic closed in the chat. Currently holds no information.
/// <https://core.telegram.org/bots/api#forumtopicclosed>
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForumTopicClosed {}
