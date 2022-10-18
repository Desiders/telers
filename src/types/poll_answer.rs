use super::User;

use serde::{Deserialize, Serialize};

/// This object represents an answer of a user in a non-anonymous poll.
/// <https://core.telegram.org/bots/api#pollanswer>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PollAnswer {
    /// Unique poll identifier
    pub poll_id: String,
    /// The user, who changed the answer to the poll
    pub user: User,
    /// 0-based identifiers of answer options, chosen by the user. May be empty if the user retracted their vote.
    pub option_ids: Vec<i64>,
}

impl Default for PollAnswer {
    fn default() -> Self {
        Self {
            poll_id: String::default(),
            user: User::default(),
            option_ids: Vec::default(),
        }
    }
}
