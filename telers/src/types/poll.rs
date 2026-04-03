use serde::{Deserialize, Serialize};
/// This object contains information about a poll.
/// Currently, it can be one of
/// - [`crate::types::PollQuiz`]
/// - [`crate::types::PollRegular`]
/// # Documentation
/// <https://core.telegram.org/bots/api#poll>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Poll {
    Regular(crate::types::PollRegular),
    Quiz(crate::types::PollQuiz),
}
impl Poll {
    /// Helper method for field `allows_multiple_answers`.
    ///
    /// `true`, if the poll allows multiple answers
    #[must_use]
    pub fn allows_multiple_answers(&self) -> bool {
        match self {
            Self::Regular(val) => val.allows_multiple_answers,
            Self::Quiz(val) => val.allows_multiple_answers,
        }
    }

    /// Helper method for field `allows_revoting`.
    ///
    /// `true`, if the poll allows to change the chosen answer options
    #[must_use]
    pub fn allows_revoting(&self) -> bool {
        match self {
            Self::Regular(val) => val.allows_revoting,
            Self::Quiz(val) => val.allows_revoting,
        }
    }

    /// Helper method for field `close_date`.
    ///
    /// Point in time (Unix timestamp) when the poll will be automatically closed
    #[must_use]
    pub fn close_date(&self) -> Option<i64> {
        match self {
            Self::Regular(val) => val.close_date,
            Self::Quiz(val) => val.close_date,
        }
    }

    /// Helper method for field `correct_option_ids`.
    ///
    /// Array of 0-based identifiers of the correct answer options. Available only for polls in quiz mode which are closed or were sent (not forwarded) by the bot or to the private chat with the bot.
    #[must_use]
    pub fn correct_option_ids(&self) -> Option<&[i64]> {
        match self {
            Self::Quiz(val) => val.correct_option_ids.as_deref(),
            Self::Regular(_) => None,
        }
    }

    /// Helper method for field `description`.
    ///
    /// Description of the poll; for polls inside the Message object only
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        match self {
            Self::Regular(val) => val.description.as_deref(),
            Self::Quiz(val) => val.description.as_deref(),
        }
    }

    /// Helper method for field `description_entities`.
    ///
    /// Special entities like usernames, URLs, bot commands, etc. that appear in the description
    #[must_use]
    pub fn description_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Regular(val) => val.description_entities.as_deref(),
            Self::Quiz(val) => val.description_entities.as_deref(),
        }
    }

    /// Helper method for field `explanation`.
    ///
    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters
    #[must_use]
    pub fn explanation(&self) -> Option<&str> {
        match self {
            Self::Quiz(val) => val.explanation.as_deref(),
            Self::Regular(_) => None,
        }
    }

    /// Helper method for field `explanation_entities`.
    ///
    /// Special entities like usernames, URLs, bot commands, etc. that appear in the explanation
    #[must_use]
    pub fn explanation_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Regular(val) => val.explanation_entities.as_deref(),
            Self::Quiz(val) => val.explanation_entities.as_deref(),
        }
    }

    /// Helper method for field `id`.
    ///
    /// Unique poll identifier
    #[must_use]
    pub fn id(&self) -> &str {
        match self {
            Self::Regular(val) => val.id.as_ref(),
            Self::Quiz(val) => val.id.as_ref(),
        }
    }

    /// Helper method for field `is_anonymous`.
    ///
    /// `true`, if the poll is anonymous
    #[must_use]
    pub fn is_anonymous(&self) -> bool {
        match self {
            Self::Regular(val) => val.is_anonymous,
            Self::Quiz(val) => val.is_anonymous,
        }
    }

    /// Helper method for field `is_closed`.
    ///
    /// `true`, if the poll is closed
    #[must_use]
    pub fn is_closed(&self) -> bool {
        match self {
            Self::Regular(val) => val.is_closed,
            Self::Quiz(val) => val.is_closed,
        }
    }

    /// Helper method for field `open_period`.
    ///
    /// Amount of time in seconds the poll will be active after creation
    #[must_use]
    pub fn open_period(&self) -> Option<i64> {
        match self {
            Self::Regular(val) => val.open_period,
            Self::Quiz(val) => val.open_period,
        }
    }

    /// Helper method for field `options`.
    ///
    /// List of poll options
    #[must_use]
    pub fn options(&self) -> &[crate::types::PollOption] {
        match self {
            Self::Regular(val) => val.options.as_ref(),
            Self::Quiz(val) => val.options.as_ref(),
        }
    }

    /// Helper method for field `question`.
    ///
    /// Poll question, 1-300 characters
    #[must_use]
    pub fn question(&self) -> &str {
        match self {
            Self::Regular(val) => val.question.as_ref(),
            Self::Quiz(val) => val.question.as_ref(),
        }
    }

    /// Helper method for field `question_entities`.
    ///
    /// Special entities that appear in the question. Currently, only custom emoji entities are allowed in poll questions
    #[must_use]
    pub fn question_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Regular(val) => val.question_entities.as_deref(),
            Self::Quiz(val) => val.question_entities.as_deref(),
        }
    }

    /// Helper method for field `total_voter_count`.
    ///
    /// Total number of users that voted in the poll
    #[must_use]
    pub fn total_voter_count(&self) -> i64 {
        match self {
            Self::Regular(val) => val.total_voter_count,
            Self::Quiz(val) => val.total_voter_count,
        }
    }
}
impl From<crate::types::PollRegular> for Poll {
    fn from(val: crate::types::PollRegular) -> Self {
        Self::Regular(val)
    }
}
impl TryFrom<Poll> for crate::types::PollRegular {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Poll) -> Result<Self, Self::Error> {
        match val {
            Poll::Regular(inner) => Ok(inner),
            Poll::Quiz(_) => Err(Self::Error::new(stringify!(Poll), stringify!(PollRegular))),
        }
    }
}
impl From<crate::types::PollQuiz> for Poll {
    fn from(val: crate::types::PollQuiz) -> Self {
        Self::Quiz(val)
    }
}
impl TryFrom<Poll> for crate::types::PollQuiz {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Poll) -> Result<Self, Self::Error> {
        match val {
            Poll::Quiz(inner) => Ok(inner),
            Poll::Regular(_) => Err(Self::Error::new(stringify!(Poll), stringify!(PollQuiz))),
        }
    }
}
