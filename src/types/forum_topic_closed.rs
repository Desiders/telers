use serde::Deserialize;

/// This object represents a service message about a forum topic closed in the chat. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#forumtopicclosed>
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize)]
pub struct ForumTopicClosed {}
