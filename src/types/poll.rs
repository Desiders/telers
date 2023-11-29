use super::{MessageEntity, PollOption, Update, UpdateKind};

use crate::errors::ConvertToTypeError;

use serde::Deserialize;

/// This object contains information about a poll.
/// # Documentation
/// <https://core.telegram.org/bots/api#poll>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Poll {
    Regular(Regular),
    Quiz(Quiz),
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct Regular {
    /// Unique poll identifier
    pub id: Box<str>,
    /// Poll question, 1-300 characters
    pub question: Box<str>,
    /// List of poll options
    pub options: Box<[PollOption]>,
    /// Total number of users that voted in the poll
    pub total_voter_count: i64,
    /// `true`, if the poll is closed
    pub is_closed: bool,
    /// `true`, if the poll is anonymous
    pub is_anonymous: bool,
    /// `true`, if the poll allows multiple answers
    pub allows_multiple_answers: bool,
    /// Amount of time in seconds the poll will be active after creation
    pub open_period: Option<i64>,
    /// Point in time (Unix timestamp) when the poll will be automatically closed
    pub close_date: Option<i64>,
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct Quiz {
    /// Unique poll identifier
    pub id: Box<str>,
    /// Poll question, 1-300 characters
    pub question: Box<str>,
    /// List of poll options
    pub options: Box<[PollOption]>,
    /// Total number of users that voted in the poll
    pub total_voter_count: i64,
    /// `true`, if the poll is closed
    pub is_closed: bool,
    /// `true`, if the poll is anonymous
    pub is_anonymous: bool,
    /// `true`, if the poll allows multiple answers
    pub allows_multiple_answers: bool,
    /// 0-based identifier of the correct answer option. Available only for polls in the quiz mode, which are closed, or was sent (not forwarded) by the bot or to the private chat with the bot.
    pub correct_option_id: Option<i64>,
    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters
    pub explanation: Option<Box<str>>,
    /// Special entities like usernames, URLs, bot commands, etc. that appear in the explanation
    pub explanation_entities: Option<Box<[MessageEntity]>>,
    /// Amount of time in seconds the poll will be active after creation
    pub open_period: Option<i64>,
    /// Point in time (Unix timestamp) when the poll will be automatically closed
    pub close_date: Option<i64>,
}

impl Default for Poll {
    fn default() -> Self {
        Self::Regular(Regular::default())
    }
}

impl TryFrom<Poll> for Regular {
    type Error = ConvertToTypeError;

    fn try_from(poll: Poll) -> Result<Self, Self::Error> {
        if let Poll::Regular(val) = poll {
            Ok(val)
        } else {
            Err(ConvertToTypeError::new("Poll", "Regular"))
        }
    }
}

impl TryFrom<Poll> for Quiz {
    type Error = ConvertToTypeError;

    fn try_from(poll: Poll) -> Result<Self, Self::Error> {
        if let Poll::Quiz(val) = poll {
            Ok(val)
        } else {
            Err(ConvertToTypeError::new("Poll", "Quiz"))
        }
    }
}

impl TryFrom<Update> for Poll {
    type Error = ConvertToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        match update.kind {
            UpdateKind::Poll(val) => Ok(val),
            _ => Err(ConvertToTypeError::new("Update", "Poll")),
        }
    }
}

impl TryFrom<Update> for Regular {
    type Error = ConvertToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        Poll::try_from(update)?.try_into()
    }
}

impl TryFrom<Update> for Quiz {
    type Error = ConvertToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        Poll::try_from(update)?.try_into()
    }
}
