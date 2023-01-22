use serde::Deserialize;

/// This object represents a service message about a forum topic closed in the chat. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#forumtopicclosed>
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct ForumTopicClosed {}
