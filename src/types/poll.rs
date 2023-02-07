use super::{MessageEntity, PollOption, Update};

use crate::error::ConvertUpdateToTypeError;

use serde::Deserialize;

/// This object contains information about a poll.
/// # Documentation
/// <https://core.telegram.org/bots/api#poll>
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct Poll {
    /// Unique poll identifier
    pub id: String,
    /// Poll question, 1-300 characters
    pub question: String,
    /// List of poll options
    pub options: Vec<PollOption>,
    /// Total number of users that voted in the poll
    pub total_voter_count: i64,
    /// `True`, if the poll is closed
    pub is_closed: bool,
    /// `True`, if the poll is anonymous
    pub is_anonymous: bool,
    /// Poll type, currently can be 'regular' or 'quiz'
    #[serde(rename = "type")]
    pub poll_type: String,
    /// `True`, if the poll allows multiple answers
    pub allows_multiple_answers: bool,
    /// *Optional*. 0-based identifier of the correct answer option. Available only for polls in the quiz mode, which are closed, or was sent (not forwarded) by the bot or to the private chat with the bot.
    pub correct_option_id: Option<i64>,
    /// *Optional*. Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters
    pub explanation: Option<String>,
    /// *Optional*. Special entities like usernames, URLs, bot commands, etc. that appear in the *explanation*
    pub explanation_entities: Option<Vec<MessageEntity>>,
    /// *Optional*. Amount of time in seconds the poll will be active after creation
    pub open_period: Option<i64>,
    /// *Optional*. Point in time (Unix timestamp) when the poll will be automatically closed
    pub close_date: Option<i64>,
}

impl TryFrom<Update> for Poll {
    type Error = ConvertUpdateToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        if let Some(poll) = update.poll {
            Ok(poll)
        } else {
            Err(ConvertUpdateToTypeError::new(format!(
                "Update {update:?} doesn't contain `Poll`"
            )))
        }
    }
}
