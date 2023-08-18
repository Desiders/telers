use super::{Chat, Update, User};

use crate::errors::ConvertUpdateToTypeError;

use serde::Deserialize;

/// This object represents an answer of a user in a non-anonymous poll.
/// # Documentation
/// <https://core.telegram.org/bots/api#pollanswer>
#[derive(Default, Clone, Debug, PartialEq, Deserialize)]
pub struct PollAnswer {
    /// Unique poll identifier
    pub poll_id: String,
    /// The chat that changed the answer to the poll, if the voter is anonymous
    pub voter_chat: Option<Chat>,
    /// The user that changed the answer to the poll, if the voter isn't anonymous
    pub user: Option<User>,
    /// 0-based identifiers of answer options, chosen by the user. May be empty if the user retracted their vote.
    pub option_ids: Vec<i64>,
}

impl PollAnswer {
    /// Gets the sender chat ID from the poll answer
    #[must_use]
    pub const fn sender_chat_id(&self) -> Option<i64> {
        if let Some(chat) = &self.voter_chat {
            Some(chat.id)
        } else {
            None
        }
    }

    /// Gets the sender user ID from the poll answer
    #[must_use]
    pub const fn sender_user_id(&self) -> Option<i64> {
        if let Some(user) = &self.user {
            Some(user.id)
        } else {
            None
        }
    }

    /// Gets the sender user ID from the poll answer
    /// # Notes
    /// Alias to `sender_user_id` method
    #[must_use]
    pub const fn user_id(&self) -> Option<i64> {
        self.sender_user_id()
    }
}

impl TryFrom<Update> for PollAnswer {
    type Error = ConvertUpdateToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        if let Some(poll_answer) = update.poll_answer {
            Ok(poll_answer)
        } else {
            Err(ConvertUpdateToTypeError::new("PollAnswer"))
        }
    }
}
