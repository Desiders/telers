use crate::client::Bot;
use serde::Serialize;
/// Marks incoming message as read on behalf of a business account. Requires the `can_read_messages` business bot right. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#readbusinessmessage>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct ReadBusinessMessage {
    /// Unique identifier of the business connection on behalf of which to read the message
    pub business_connection_id: Box<str>,
    /// Unique identifier of the chat in which the message was received. The chat must have been active in the last 24 hours.
    pub chat_id: i64,
    /// Unique identifier of the message to mark as read
    pub message_id: i64,
}
impl ReadBusinessMessage {
    /// Creates a new `ReadBusinessMessage`.
    ///
    /// # Arguments
    /// * `business_connection_id` - Unique identifier of the business connection on behalf of which to read the message
    /// * `chat_id` - Unique identifier of the chat in which the message was received. The chat must have been active in the last 24 hours.
    /// * `message_id` - Unique identifier of the message to mark as read
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<i64>, T2: Into<i64>>(
        business_connection_id: T0,
        chat_id: T1,
        message_id: T2,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            chat_id: chat_id.into(),
            message_id: message_id.into(),
        }
    }

    /// Unique identifier of the business connection on behalf of which to read the message
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.business_connection_id = val.into();
        this
    }

    /// Unique identifier of the chat in which the message was received. The chat must have been active in the last 24 hours.
    #[must_use]
    pub fn chat_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// Unique identifier of the message to mark as read
    #[must_use]
    pub fn message_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.message_id = val.into();
        this
    }
}
impl super::TelegramMethod for ReadBusinessMessage {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("readBusinessMessage", self, None)
    }
}
