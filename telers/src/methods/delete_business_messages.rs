use crate::client::Bot;
use serde::Serialize;
/// Delete messages on behalf of a business account. Requires the `can_delete_sent_messages` business bot right to delete messages sent by the bot itself, or the `can_delete_all_messages` business bot right to delete any message. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#deletebusinessmessages>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct DeleteBusinessMessages {
    /// Unique identifier of the business connection on behalf of which to delete the messages
    pub business_connection_id: Box<str>,
    /// A JSON-serialized list of 1-100 identifiers of messages to delete. All messages must be from the same chat. See [`crate::methods::DeleteMessage`] for limitations on which messages can be deleted
    pub message_ids: Box<[u8]>,
}
impl DeleteBusinessMessages {
    /// Creates a new `DeleteBusinessMessages`.
    ///
    /// # Arguments
    /// * `business_connection_id` - Unique identifier of the business connection on behalf of which to delete the messages
    /// * `message_ids` - A JSON-serialized list of 1-100 identifiers of messages to delete. All messages must be from the same chat. See [`crate::methods::DeleteMessage`] for limitations on which messages can be deleted
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1Item: Into<u8>, T1: IntoIterator<Item = T1Item>>(
        business_connection_id: T0,
        message_ids: T1,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            message_ids: message_ids.into_iter().map(Into::into).collect(),
        }
    }

    /// Unique identifier of the business connection on behalf of which to delete the messages
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.business_connection_id = val.into();
        this
    }

    /// A JSON-serialized list of 1-100 identifiers of messages to delete. All messages must be from the same chat. See [`crate::methods::DeleteMessage`] for limitations on which messages can be deleted
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn message_ids<TItem: Into<u8>, T: IntoIterator<Item = TItem>>(self, val: T) -> Self {
        let mut this = self;
        this.message_ids = this
            .message_ids
            .into_vec()
            .into_iter()
            .chain(val.into_iter().map(Into::into))
            .collect();
        this
    }

    /// A JSON-serialized list of 1-100 identifiers of messages to delete. All messages must be from the same chat. See [`crate::methods::DeleteMessage`] for limitations on which messages can be deleted
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn message_id<T: Into<u8>>(self, val: T) -> Self {
        let mut this = self;
        this.message_ids = this
            .message_ids
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }
}
impl super::TelegramMethod for DeleteBusinessMessages {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("deleteBusinessMessages", self, None)
    }
}
