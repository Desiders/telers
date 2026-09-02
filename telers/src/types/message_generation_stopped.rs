use serde::{Deserialize, Serialize};
/// This object describes an update about a user stopping message generation.
/// # Documentation
/// <https://core.telegram.org/bots/api#messagegenerationstopped>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageGenerationStopped {
    /// Chat in which the message is generated
    pub chat: Box<crate::types::Chat>,
    /// Unique identifier of the message thread in which the message is generated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Unique identifier of the message draft which was stopped
    pub draft_id: i64,
}
impl MessageGenerationStopped {
    /// Creates a new `MessageGenerationStopped`.
    ///
    /// # Arguments
    /// * `chat` - Chat in which the message is generated
    /// * `draft_id` - Unique identifier of the message draft which was stopped
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::Chat>, T1: Into<i64>>(chat: T0, draft_id: T1) -> Self {
        Self {
            chat: Box::new(chat.into()),
            message_thread_id: None,
            draft_id: draft_id.into(),
        }
    }

    /// Chat in which the message is generated
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(mut self, val: T) -> Self {
        self.chat = Box::new(val.into());
        self
    }

    /// Unique identifier of the message thread in which the message is generated
    #[must_use]
    pub fn message_thread_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_thread_id = Some(val.into());
        self
    }

    /// Unique identifier of the message thread in which the message is generated
    #[must_use]
    pub fn message_thread_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.message_thread_id = val.map(Into::into);
        self
    }

    /// Unique identifier of the message draft which was stopped
    #[must_use]
    pub fn draft_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.draft_id = val.into();
        self
    }
}
