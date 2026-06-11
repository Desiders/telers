use serde::{Deserialize, Serialize};
/// Describes a topic of a direct messages chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#directmessagestopic>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DirectMessagesTopic {
    /// Unique identifier of the topic. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub topic_id: i64,
    /// Information about the user that created the topic. Currently, it is always present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::types::User>>,
}
impl DirectMessagesTopic {
    /// Creates a new `DirectMessagesTopic`.
    ///
    /// # Arguments
    /// * `topic_id` - Unique identifier of the topic. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>>(topic_id: T0) -> Self {
        Self {
            topic_id: topic_id.into(),
            user: None,
        }
    }

    /// Unique identifier of the topic. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn topic_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.topic_id = val.into();
        self
    }

    /// Information about the user that created the topic. Currently, it is always present.
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.user = Some(Box::new(val.into()));
        self
    }

    /// Information about the user that created the topic. Currently, it is always present.
    #[must_use]
    pub fn user_option<T: Into<crate::types::User>>(mut self, val: Option<T>) -> Self {
        self.user = val.map(|val| Box::new(val.into()));
        self
    }
}
