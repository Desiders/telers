use super::{Chat, Update, UpdateKind, User};

use crate::errors::ConvertToTypeError;

use serde::Deserialize;

/// This object represents an answer of a user in a non-anonymous poll.
/// # Documentation
/// <https://core.telegram.org/bots/api#pollanswer>
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct PollAnswer {
    /// Unique poll identifier
    pub poll_id: Box<str>,
    /// The chat that changed the answer to the poll, if the voter is anonymous
    pub voter_chat: Option<Chat>,
    /// The user that changed the answer to the poll, if the voter isn't anonymous
    pub user: Option<User>,
    /// 0-based identifiers of answer options, chosen by the user. May be empty if the user retracted their vote.
    pub option_ids: Box<[i64]>,
}

impl TryFrom<Update> for PollAnswer {
    type Error = ConvertToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        match update.kind {
            UpdateKind::PollAnswer(val) => Ok(val),
            _ => Err(ConvertToTypeError::new("Update", "PollAnswer")),
        }
    }
}
