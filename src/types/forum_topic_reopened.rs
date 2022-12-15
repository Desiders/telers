use serde::{Deserialize, Serialize};

/// This object represents a service message about a forum topic reopened in the chat. Currently holds no information.
/// <https://core.telegram.org/bots/api#forumtopicreopened>
#[derive(Default, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ForumTopicReopened {}
