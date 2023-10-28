use serde::Deserialize;

/// This object represents a service message about a new forum topic created in the chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#forumtopiccreated>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct ForumTopicCreated {
    /// Name of the topic
    pub name: Box<str>,
    /// Color of the topic icon in RGB format
    pub icon_color: i64,
    /// Unique identifier of the custom emoji shown as the topic icon
    pub icon_custom_emoji_id: Box<str>,
}
