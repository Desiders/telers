use serde::{Deserialize, Serialize};

/// This object represents a service message about a new forum topic created in the chat.
/// <https://core.telegram.org/bots/api#forumtopiccreated>
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForumTopicCreated {
    /// Name of the topic
    pub name: String,
    /// Color of the topic icon in RGB format
    pub icon_color: i64,
    /// *Optional*. Unique identifier of the custom emoji shown as the topic icon
    pub icon_custom_emoji_id: String,
}
