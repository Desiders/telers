use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;

/// Use this method to delete multiple messages simultaneously. If some of the specified messages can't be found, they are skipped.
/// # Documentation
/// <https://core.telegram.org/bots/api#deletemessages>
/// # Returns
/// Returns `true` on success
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct DeleteMessages {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Identifiers of 1-100 messages to delete. See [`crate::methods::DeleteMessage`] for limitations on which messages can be deleted
    pub message_ids: Vec<i64>,
}

impl DeleteMessages {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, message_ids: impl IntoIterator<Item = i64>) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_ids: message_ids.into_iter().collect(),
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: val.into(),
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

impl TelegramMethod for DeleteMessages {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("deleteMessages", self, None)
    }
}

impl AsRef<DeleteMessages> for DeleteMessages {
    fn as_ref(&self) -> &Self {
        self
    }
}
