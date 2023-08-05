use super::{Update, User};

use crate::errors::ConvertUpdateToTypeError;

use serde::Deserialize;

/// This object represents an answer of a user in a non-anonymous poll.
/// # Documentation
/// <https://core.telegram.org/bots/api#pollanswer>
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct PollAnswer {
    /// Unique poll identifier
    pub poll_id: String,
    /// The user, who changed the answer to the poll
    pub user: User,
    /// 0-based identifiers of answer options, chosen by the user. May be empty if the user retracted their vote.
    pub option_ids: Vec<i64>,
}

impl PollAnswer {
    /// Gets the sender user ID from the poll answer
    #[must_use]
    pub const fn sender_user_id(&self) -> i64 {
        self.user.id
    }

    /// Gets the sender user ID from the poll answer
    /// # Notes
    /// Alias to `sender_user_id` method
    #[must_use]
    pub const fn user_id(&self) -> i64 {
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
