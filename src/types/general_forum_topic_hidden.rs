use serde::Deserialize;

/// This object represents a service message about General forum topic hidden in the chat. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#generalforumtopichidden>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct GeneralForumTopicHidden {}