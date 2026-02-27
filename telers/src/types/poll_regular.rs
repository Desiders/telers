use serde::{Deserialize, Serialize};
/// This object represents a regular poll.
/// # Notes
/// This object represents a poll from original poll type `regular`.
/// # Documentation
/// <https://core.telegram.org/bots/api#poll>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PollRegular {
    /// Unique poll identifier
    pub id: Box<str>,
    /// Poll question, 1-300 characters
    pub question: Box<str>,
    /// Special entities that appear in the question. Currently, only custom emoji entities are allowed in poll questions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// List of poll options
    pub options: Box<[crate::types::PollOption]>,
    /// Total number of users that voted in the poll
    pub total_voter_count: i64,
    /// `true`, if the poll is closed
    pub is_closed: bool,
    /// `true`, if the poll is anonymous
    pub is_anonymous: bool,
    /// `true`, if the poll allows multiple answers
    pub allows_multiple_answers: bool,
    /// Special entities like usernames, URLs, bot commands, etc. that appear in the explanation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// Amount of time in seconds the poll will be active after creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i64>,
    /// Point in time (Unix timestamp) when the poll will be automatically closed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,
}
impl PollRegular {
    /// Creates a new `PollRegular`.
    ///
    /// # Arguments
    /// * `id` - Unique poll identifier
    /// * `question` - Poll question, 1-300 characters
    /// * `options` - List of poll options
    /// * `total_voter_count` - Total number of users that voted in the poll
    /// * `is_closed` - `true`, if the poll is closed
    /// * `is_anonymous` - `true`, if the poll is anonymous
    /// * `allows_multiple_answers` - `true`, if the poll allows multiple answers
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<Box<str>>,
        T2Item: Into<crate::types::PollOption>,
        T2: IntoIterator<Item = T2Item>,
        T3: Into<i64>,
        T4: Into<bool>,
        T5: Into<bool>,
        T6: Into<bool>,
    >(
        id: T0,
        question: T1,
        options: T2,
        total_voter_count: T3,
        is_closed: T4,
        is_anonymous: T5,
        allows_multiple_answers: T6,
    ) -> Self {
        Self {
            id: id.into(),
            question: question.into(),
            question_entities: None,
            options: options.into_iter().map(Into::into).collect(),
            total_voter_count: total_voter_count.into(),
            is_closed: is_closed.into(),
            is_anonymous: is_anonymous.into(),
            allows_multiple_answers: allows_multiple_answers.into(),
            explanation_entities: None,
            open_period: None,
            close_date: None,
        }
    }

    /// Unique poll identifier
    #[must_use]
    pub fn id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.id = val.into();
        this
    }

    /// Poll question, 1-300 characters
    #[must_use]
    pub fn question<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.question = val.into();
        this
    }

    /// Special entities that appear in the question. Currently, only custom emoji entities are allowed in poll questions
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn question_entities<T: Into<Box<[crate::types::MessageEntity]>>>(self, val: T) -> Self {
        let mut this = self;
        this.question_entities = Some(
            this.question_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        this
    }

    /// Special entities that appear in the question. Currently, only custom emoji entities are allowed in poll questions
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn question_entity<T: Into<crate::types::MessageEntity>>(self, val: T) -> Self {
        let mut this = self;
        this.question_entities = Some(
            this.question_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// Special entities that appear in the question. Currently, only custom emoji entities are allowed in poll questions
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn question_entities_option<T: Into<Box<[crate::types::MessageEntity]>>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.question_entities = val.map(Into::into);
        this
    }

    /// List of poll options
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn options<T: Into<Box<[crate::types::PollOption]>>>(self, val: T) -> Self {
        let mut this = self;
        this.options = this
            .options
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        this
    }

    /// List of poll options
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn option<T: Into<crate::types::PollOption>>(self, val: T) -> Self {
        let mut this = self;
        this.options = this
            .options
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }

    /// Total number of users that voted in the poll
    #[must_use]
    pub fn total_voter_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.total_voter_count = val.into();
        this
    }

    /// `true`, if the poll is closed
    #[must_use]
    pub fn is_closed<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_closed = val.into();
        this
    }

    /// `true`, if the poll is anonymous
    #[must_use]
    pub fn is_anonymous<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_anonymous = val.into();
        this
    }

    /// `true`, if the poll allows multiple answers
    #[must_use]
    pub fn allows_multiple_answers<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.allows_multiple_answers = val.into();
        this
    }

    /// Special entities like usernames, URLs, bot commands, etc. that appear in the explanation
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn explanation_entities<T: Into<Box<[crate::types::MessageEntity]>>>(self, val: T) -> Self {
        let mut this = self;
        this.explanation_entities = Some(
            this.explanation_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        this
    }

    /// Special entities like usernames, URLs, bot commands, etc. that appear in the explanation
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn explanation_entity<T: Into<crate::types::MessageEntity>>(self, val: T) -> Self {
        let mut this = self;
        this.explanation_entities = Some(
            this.explanation_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// Special entities like usernames, URLs, bot commands, etc. that appear in the explanation
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn explanation_entities_option<T: Into<Box<[crate::types::MessageEntity]>>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.explanation_entities = val.map(Into::into);
        this
    }

    /// Amount of time in seconds the poll will be active after creation
    #[must_use]
    pub fn open_period<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.open_period = Some(val.into());
        this
    }

    /// Amount of time in seconds the poll will be active after creation
    #[must_use]
    pub fn open_period_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.open_period = val.map(Into::into);
        this
    }

    /// Point in time (Unix timestamp) when the poll will be automatically closed
    #[must_use]
    pub fn close_date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.close_date = Some(val.into());
        this
    }

    /// Point in time (Unix timestamp) when the poll will be automatically closed
    #[must_use]
    pub fn close_date_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.close_date = val.map(Into::into);
        this
    }
}
