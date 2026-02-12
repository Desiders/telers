use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;

/// Use this method to decline a suggested post in a direct messages chat. The bot must have the `can_manage_direct_messages` administrator right in the corresponding channel chat
/// # Documentation
/// <https://core.telegram.org/bots/api#declinesuggestedpost>
/// # Returns
/// On success, `true` is returned
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct DeclineSuggestedPost {
    /// Unique identifier for the target direct messages chat
    pub chat_id: i64,
    /// Identifier of a suggested post message to decline
    pub message_id: i64,
    /// Comment for the creator of the suggested post; 0-128 characters
    pub comment: Option<String>,
}

impl DeclineSuggestedPost {
    #[must_use]
    pub fn new(chat_id: i64, message_id: i64) -> Self {
        Self {
            chat_id,
            message_id,
            comment: None,
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

    #[must_use]
    pub fn comment(self, val: impl Into<String>) -> Self {
        Self {
            comment: Some(val.into()),
            ..self
        }
    }
}

impl DeclineSuggestedPost {
    #[must_use]
    pub fn comment_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            comment: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for DeclineSuggestedPost {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<'_, Self::Method> {
        Request::new("declineSuggestedPost", self, None)
    }
}

impl AsRef<DeclineSuggestedPost> for DeclineSuggestedPost {
    fn as_ref(&self) -> &Self {
        self
    }
}
