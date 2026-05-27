use serde::{Deserialize, Serialize};
/// This object represents a service message about an edited forum topic.
/// # Documentation
/// <https://core.telegram.org/bots/api#forumtopicedited>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForumTopicEdited {
    /// New name of the topic, if it was edited
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<str>>,
    /// New identifier of the custom emoji shown as the topic icon, if it was edited; an empty string if the icon was removed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<Box<str>>,
}
impl ForumTopicEdited {
    /// Creates a new `ForumTopicEdited`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            name: None,
            icon_custom_emoji_id: None,
        }
    }

    /// New name of the topic, if it was edited
    #[must_use]
    pub fn name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.name = Some(val.into());
        self
    }

    /// New name of the topic, if it was edited
    #[must_use]
    pub fn name_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.name = val.map(Into::into);
        self
    }

    /// New identifier of the custom emoji shown as the topic icon, if it was edited; an empty string if the icon was removed
    #[must_use]
    pub fn icon_custom_emoji_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.icon_custom_emoji_id = Some(val.into());
        self
    }

    /// New identifier of the custom emoji shown as the topic icon, if it was edited; an empty string if the icon was removed
    #[must_use]
    pub fn icon_custom_emoji_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.icon_custom_emoji_id = val.map(Into::into);
        self
    }
}
impl Default for ForumTopicEdited {
    fn default() -> Self {
        Self::new()
    }
}
