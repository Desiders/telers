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
    /// Content unknown to this version of the library
    #[serde(untagged)]
    Unknown(crate::types::PollUnknown),
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
            Self::Unknown(val) => val.allows_multiple_answers,
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
            Self::Unknown(val) => val.allows_revoting,
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
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `correct_option_ids`.
    ///
    /// Array of 0-based identifiers of the correct answer options. Available only for polls in quiz mode which are closed or were sent (not forwarded) by the bot or to the private chat with the bot.
    #[must_use]
    pub fn correct_option_ids(&self) -> Option<&[i64]> {
        match self {
            Self::Quiz(val) => val.correct_option_ids.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `country_codes`.
    ///
    /// A list of two-letter ISO 3166-1 alpha-2 country codes indicating the countries from which users can vote in the poll. The country code `FT` is used for users with anonymous numbers. If omitted, then users from any country can participate in the poll.
    #[must_use]
    pub fn country_codes(&self) -> Option<&[Box<str>]> {
        match self {
            Self::Regular(val) => val.country_codes.as_deref(),
            Self::Quiz(val) => val.country_codes.as_deref(),
            Self::Unknown(_) => None,
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
            Self::Unknown(_) => None,
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
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `explanation`.
    ///
    /// Text that is shown when a user chooses an incorrect answer or taps on the lamp icon in a quiz-style poll, 0-200 characters
    #[must_use]
    pub fn explanation(&self) -> Option<&str> {
        match self {
            Self::Quiz(val) => val.explanation.as_deref(),
            _ => None,
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
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `explanation_media`.
    ///
    /// Media added to the quiz explanation
    #[must_use]
    pub fn explanation_media(&self) -> Option<&crate::types::PollMedia> {
        match self {
            Self::Quiz(val) => val.explanation_media.as_ref(),
            _ => None,
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
            Self::Unknown(val) => val.id.as_ref(),
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
            Self::Unknown(val) => val.is_anonymous,
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
            Self::Unknown(val) => val.is_closed,
        }
    }

    /// Helper method for field `media`.
    ///
    /// Media added to the poll description; for polls inside the Message object only
    #[must_use]
    pub fn media(&self) -> Option<&crate::types::PollMedia> {
        match self {
            Self::Regular(val) => val.media.as_ref(),
            Self::Quiz(val) => val.media.as_ref(),
            Self::Unknown(_) => None,
        }
    }

    /// Helper method for field `members_only`.
    ///
    /// `true` if voting is limited to users who have been members of the chat where the poll was originally sent for more than 24 hours
    #[must_use]
    pub fn members_only(&self) -> bool {
        match self {
            Self::Regular(val) => val.members_only,
            Self::Quiz(val) => val.members_only,
            Self::Unknown(val) => val.members_only,
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
            Self::Unknown(_) => None,
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
            Self::Unknown(val) => val.options.as_ref(),
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
            Self::Unknown(val) => val.question.as_ref(),
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
            Self::Unknown(_) => None,
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
            Self::Unknown(val) => val.total_voter_count,
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
        if let Poll::Regular(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(stringify!(Poll), stringify!(PollRegular)))
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
        if let Poll::Quiz(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(stringify!(Poll), stringify!(PollQuiz)))
        }
    }
}
impl From<crate::types::PollUnknown> for Poll {
    fn from(val: crate::types::PollUnknown) -> Self {
        Self::Unknown(val)
    }
}
impl TryFrom<Poll> for crate::types::PollUnknown {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Poll) -> Result<Self, Self::Error> {
        if let Poll::Unknown(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(stringify!(Poll), stringify!(PollUnknown)))
        }
    }
}
