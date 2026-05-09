use serde::{Deserialize, Serialize};
/// This object represents a forum topic.
/// # Documentation
/// <https://core.telegram.org/bots/api#forumtopic>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForumTopic {
    /// Unique identifier of the forum topic
    pub message_thread_id: i64,
    /// Name of the topic
    pub name: Box<str>,
    /// Color of the topic icon in RGB format
    pub icon_color: i64,
    /// Unique identifier of the custom emoji shown as the topic icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<Box<str>>,
    /// `true`, if the name of the topic wasn't specified explicitly by its creator and likely needs to be changed by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_name_implicit: Option<bool>,
}
impl ForumTopic {
    /// Creates a new `ForumTopic`.
    ///
    /// # Arguments
    /// * `message_thread_id` - Unique identifier of the forum topic
    /// * `name` - Name of the topic
    /// * `icon_color` - Color of the topic icon in RGB format
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<Box<str>>, T2: Into<i64>>(
        message_thread_id: T0,
        name: T1,
        icon_color: T2,
    ) -> Self {
        Self {
            message_thread_id: message_thread_id.into(),
            name: name.into(),
            icon_color: icon_color.into(),
            icon_custom_emoji_id: None,
            is_name_implicit: None,
        }
    }

    /// Unique identifier of the forum topic
    #[must_use]
    pub fn message_thread_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_thread_id = val.into();
        self
    }

    /// Name of the topic
    #[must_use]
    pub fn name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.name = val.into();
        self
    }

    /// Color of the topic icon in RGB format
    #[must_use]
    pub fn icon_color<T: Into<i64>>(mut self, val: T) -> Self {
        self.icon_color = val.into();
        self
    }

    /// Unique identifier of the custom emoji shown as the topic icon
    #[must_use]
    pub fn icon_custom_emoji_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.icon_custom_emoji_id = Some(val.into());
        self
    }

    /// Unique identifier of the custom emoji shown as the topic icon
    #[must_use]
    pub fn icon_custom_emoji_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.icon_custom_emoji_id = val.map(Into::into);
        self
    }

    /// `true`, if the name of the topic wasn't specified explicitly by its creator and likely needs to be changed by the bot
    #[must_use]
    pub fn is_name_implicit<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_name_implicit = Some(val.into());
        self
    }

    /// `true`, if the name of the topic wasn't specified explicitly by its creator and likely needs to be changed by the bot
    #[must_use]
    pub fn is_name_implicit_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_name_implicit = val.map(Into::into);
        self
    }
}
