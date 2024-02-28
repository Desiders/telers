use super::Chat;

use serde::Deserialize;

/// This object represents a message about a forwarded story in the chat. Currently holds no information.
/// # Documentation
/// <https://core.telegram.org/bots/api#story>
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct Story {
    /// Chat that posted the story
    pub chat: Chat,
    /// Unique identifier for the story in the chat
    pub id: i64,
}
