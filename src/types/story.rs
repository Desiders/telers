use serde::Deserialize;

/// This object represents a message about a forwarded story in the chat. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#story>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct Story;
