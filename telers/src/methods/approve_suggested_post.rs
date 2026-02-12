use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;

/// Use this method to approve a suggested post in a direct messages chat. The bot must have the `can_post_messages` administrator right in the corresponding channel chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#approvesuggestedpost>
/// # Returns
/// On success, `true` is returned
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct ApproveSuggestedPost {
    /// Unique identifier for the target direct messages chat
    pub chat_id: i64,
    /// Identifier of a suggested post message to approve
    pub message_id: i64,
    /// Point in time (Unix timestamp) when the post is expected to be published; omit if the date has already been specified when the suggested post was created. If specified, then the date must be not more than 2678400 seconds (30 days) in the future
    pub send_date: Option<i64>,
}

impl ApproveSuggestedPost {
    #[must_use]
    pub fn new(chat_id: i64, message_id: i64) -> Self {
        Self {
            chat_id,
            message_id,
            send_date: None,
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
    pub fn send_date(self, val: i64) -> Self {
        Self {
            send_date: Some(val),
            ..self
        }
    }
}

impl ApproveSuggestedPost {
    #[must_use]
    pub fn send_date_option(self, val: Option<i64>) -> Self {
        Self {
            send_date: val,
            ..self
        }
    }
}

impl TelegramMethod for ApproveSuggestedPost {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("approveSuggestedPost", self, None)
    }
}

impl AsRef<ApproveSuggestedPost> for ApproveSuggestedPost {
    fn as_ref(&self) -> &Self {
        self
    }
}
