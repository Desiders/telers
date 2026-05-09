use serde::{Deserialize, Serialize};
/// This object is received when messages are deleted from a connected business account.
/// # Documentation
/// <https://core.telegram.org/bots/api#businessmessagesdeleted>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BusinessMessagesDeleted {
    /// Unique identifier of the business connection
    pub business_connection_id: Box<str>,
    /// Information about a chat in the business account. The bot may not have access to the chat or the corresponding user.
    pub chat: Box<crate::types::Chat>,
    /// The list of identifiers of deleted messages in the chat of the business account
    pub message_ids: Box<[i64]>,
}
impl BusinessMessagesDeleted {
    /// Creates a new `BusinessMessagesDeleted`.
    ///
    /// # Arguments
    /// * `business_connection_id` - Unique identifier of the business connection
    /// * `chat` - Information about a chat in the business account. The bot may not have access to the chat or the corresponding user.
    /// * `message_ids` - The list of identifiers of deleted messages in the chat of the business account
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<crate::types::Chat>,
        T2Item: Into<i64>,
        T2: IntoIterator<Item = T2Item>,
    >(
        business_connection_id: T0,
        chat: T1,
        message_ids: T2,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            chat: Box::new(chat.into()),
            message_ids: message_ids.into_iter().map(Into::into).collect(),
        }
    }

    /// Unique identifier of the business connection
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.business_connection_id = val.into();
        self
    }

    /// Information about a chat in the business account. The bot may not have access to the chat or the corresponding user.
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(mut self, val: T) -> Self {
        self.chat = Box::new(val.into());
        self
    }

    /// The list of identifiers of deleted messages in the chat of the business account
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn message_ids<T: Into<Box<[i64]>>>(mut self, val: T) -> Self {
        self.message_ids = self
            .message_ids
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// The list of identifiers of deleted messages in the chat of the business account
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_ids = self
            .message_ids
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }
}
