use super::{Update, User};

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

impl From<Update> for PollAnswer {
    fn from(update: Update) -> Self {
        update.poll_answer.expect("Update isn't a `PollAnswer`")
    }
}
