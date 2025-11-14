use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;

/// Marks incoming message as read on behalf of a business account. Requires the `can_read_messages` business bot right.
/// # Documentation
/// <https://core.telegram.org/bots/api#readbusinessmessage>
/// # Returns
/// On success, `true` is returned
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct ReadBusinessMessage {
    /// Unique identifier of the business connection on behalf of which to read the message
    pub business_connection_id: String,
    /// Unique identifier of the chat in which the message was received. The chat must have been active in the last 24 hours.
    pub chat_id: i64,
    /// Unique identifier of the message to mark as read
    pub message_id: i64,
}

impl ReadBusinessMessage {
    #[must_use]
    pub fn new(business_connection_id: impl Into<String>, chat_id: i64, message_id: i64) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            chat_id,
            message_id,
        }
    }

    #[must_use]
    pub fn business_connection_id(self, val: impl Into<String>) -> Self {
        Self {
            business_connection_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn chat_id(self, val: i64) -> Self {
        Self {
            chat_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn message_id(self, val: i64) -> Self {
        Self {
            message_id: val,
            ..self
        }
    }
}

impl TelegramMethod for ReadBusinessMessage {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<'_, Self::Method> {
        Request::new("readBusinessMessage", self, None)
    }
}

impl AsRef<ReadBusinessMessage> for ReadBusinessMessage {
    fn as_ref(&self) -> &Self {
        self
    }
}
