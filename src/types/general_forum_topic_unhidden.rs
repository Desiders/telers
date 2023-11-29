use serde::Deserialize;

/// This object represents a service message about General forum topic unhidden in the chat. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#generalforumtopicunhidden>
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize)]
pub struct GeneralForumTopicUnhidden {}
