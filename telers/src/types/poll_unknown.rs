use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// This object represents a Poll unknown to this version of the library.
/// # Notes
/// Fields shared by all known variants are parsed as usual; everything else is kept in `extra`, so the object can be inspected and reserialized without data loss.
/// # Documentation
/// <https://core.telegram.org/bots/api#poll>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PollUnknown {
    /// Raw `type` value of the variant unknown to this version of the library
    pub r#type: Box<str>,
    /// Unique poll identifier
    pub id: Box<str>,
    /// Poll question, 1-300 characters
    pub question: Box<str>,
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
    /// `true`, if the poll allows to change the chosen answer options
    pub allows_revoting: bool,
    /// `true` if voting is limited to users who have been members of the chat where the poll was originally sent for more than 24 hours
    #[serde(default)]
    pub members_only: bool,
    #[serde(flatten)]
    pub extra: BTreeMap<Box<str>, serde_json::Value>,
}
impl PollUnknown {
    /// Creates a new `PollUnknown`.
    ///
    /// # Arguments
    /// * `type` - Raw `type` value of the variant unknown to this version of the library
    /// * `id` - Unique poll identifier
    /// * `question` - Poll question, 1-300 characters
    /// * `options` - List of poll options
    /// * `total_voter_count` - Total number of users that voted in the poll
    /// * `is_closed` - `true`, if the poll is closed
    /// * `is_anonymous` - `true`, if the poll is anonymous
    /// * `allows_multiple_answers` - `true`, if the poll allows multiple answers
    /// * `allows_revoting` - `true`, if the poll allows to change the chosen answer options
    /// * `members_only` - `true` if voting is limited to users who have been members of the chat where the poll was originally sent for more than 24 hours
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<Box<str>>,
        T2: Into<Box<str>>,
        T3Item: Into<crate::types::PollOption>,
        T3: IntoIterator<Item = T3Item>,
        T4: Into<i64>,
        T5: Into<bool>,
        T6: Into<bool>,
        T7: Into<bool>,
        T8: Into<bool>,
        T9: Into<bool>,
    >(
        r#type: T0,
        id: T1,
        question: T2,
        options: T3,
        total_voter_count: T4,
        is_closed: T5,
        is_anonymous: T6,
        allows_multiple_answers: T7,
        allows_revoting: T8,
        members_only: T9,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            id: id.into(),
            question: question.into(),
            options: options.into_iter().map(Into::into).collect(),
            total_voter_count: total_voter_count.into(),
            is_closed: is_closed.into(),
            is_anonymous: is_anonymous.into(),
            allows_multiple_answers: allows_multiple_answers.into(),
            allows_revoting: allows_revoting.into(),
            members_only: members_only.into(),
            extra: BTreeMap::new(),
        }
    }

    /// Raw `type` value of the variant unknown to this version of the library
    #[must_use]
    pub fn r#type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.r#type = val.into();
        self
    }

    /// Unique poll identifier
    #[must_use]
    pub fn id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// Poll question, 1-300 characters
    #[must_use]
    pub fn question<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.question = val.into();
        self
    }

    /// List of poll options
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn options<T: Into<Box<[crate::types::PollOption]>>>(mut self, val: T) -> Self {
        self.options = self
            .options
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// List of poll options
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn option<T: Into<crate::types::PollOption>>(mut self, val: T) -> Self {
        self.options = self
            .options
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// Total number of users that voted in the poll
    #[must_use]
    pub fn total_voter_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.total_voter_count = val.into();
        self
    }

    /// `true`, if the poll is closed
    #[must_use]
    pub fn is_closed<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_closed = val.into();
        self
    }

    /// `true`, if the poll is anonymous
    #[must_use]
    pub fn is_anonymous<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_anonymous = val.into();
        self
    }

    /// `true`, if the poll allows multiple answers
    #[must_use]
    pub fn allows_multiple_answers<T: Into<bool>>(mut self, val: T) -> Self {
        self.allows_multiple_answers = val.into();
        self
    }

    /// `true`, if the poll allows to change the chosen answer options
    #[must_use]
    pub fn allows_revoting<T: Into<bool>>(mut self, val: T) -> Self {
        self.allows_revoting = val.into();
        self
    }

    /// `true` if voting is limited to users who have been members of the chat where the poll was originally sent for more than 24 hours
    #[must_use]
    pub fn members_only<T: Into<bool>>(mut self, val: T) -> Self {
        self.members_only = val.into();
        self
    }
}
