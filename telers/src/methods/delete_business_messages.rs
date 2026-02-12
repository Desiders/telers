use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;

/// Delete messages on behalf of a business account. Requires the `can_delete_sent_messages` business bot right to delete messages sent by the bot itself, or the `can_delete_all_messages` business bot right to delete any message.
/// # Documentation
/// <https://core.telegram.org/bots/api#deletebusinessmessages>
/// # Returns
/// On success, `true` is returned
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct DeleteBusinessMessages {
    /// Unique identifier of the business connection on behalf of which to delete the messages
    pub business_connection_id: String,
    /// A JSON-serialized list of 1-100 identifiers of messages to delete. All messages must be from the same chat. See [`DeleteMessage`] for limitations on which messages can be deleted
    pub message_ids: Vec<i64>,
}

impl DeleteBusinessMessages {
    #[must_use]
    pub fn new(
        business_connection_id: impl Into<String>,
        message_ids: impl IntoIterator<Item = i64>,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            message_ids: message_ids.into_iter().collect(),
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
    pub fn message_id(self, val: i64) -> Self {
        Self {
            message_ids: self.message_ids.into_iter().chain(Some(val)).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn message_ids(self, val: impl IntoIterator<Item = i64>) -> Self {
        Self {
            message_ids: self.message_ids.into_iter().chain(val).collect(),
            ..self
        }
    }
}

impl TelegramMethod for DeleteBusinessMessages {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("deleteBusinessMessages", self, None)
    }
}

impl AsRef<DeleteBusinessMessages> for DeleteBusinessMessages {
    fn as_ref(&self) -> &Self {
        self
    }
}
