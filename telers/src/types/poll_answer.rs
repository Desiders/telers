use serde::{Deserialize, Serialize};
/// This object represents an answer of a user in a non-anonymous poll.
/// # Documentation
/// <https://core.telegram.org/bots/api#pollanswer>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PollAnswer {
    /// Unique poll identifier
    pub poll_id: Box<str>,
    /// The chat that changed the answer to the poll, if the voter is anonymous
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voter_chat: Option<Box<crate::types::Chat>>,
    /// The user that changed the answer to the poll, if the voter isn't anonymous
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::types::User>>,
    /// 0-based identifiers of chosen answer options. May be empty if the vote was retracted.
    pub option_ids: Box<[i64]>,
    /// Persistent identifiers of the chosen answer options. May be empty if the vote was retracted.
    pub option_persistent_ids: Box<[Box<str>]>,
}
impl PollAnswer {
    /// Creates a new `PollAnswer`.
    ///
    /// # Arguments
    /// * `poll_id` - Unique poll identifier
    /// * `option_ids` - 0-based identifiers of chosen answer options. May be empty if the vote was retracted.
    /// * `option_persistent_ids` - Persistent identifiers of the chosen answer options. May be empty if the vote was retracted.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1Item: Into<i64>,
        T1: IntoIterator<Item = T1Item>,
        T2Item: Into<Box<str>>,
        T2: IntoIterator<Item = T2Item>,
    >(
        poll_id: T0,
        option_ids: T1,
        option_persistent_ids: T2,
    ) -> Self {
        Self {
            poll_id: poll_id.into(),
            voter_chat: None,
            user: None,
            option_ids: option_ids.into_iter().map(Into::into).collect(),
            option_persistent_ids: option_persistent_ids.into_iter().map(Into::into).collect(),
        }
    }

    /// Unique poll identifier
    #[must_use]
    pub fn poll_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.poll_id = val.into();
        self
    }

    /// The chat that changed the answer to the poll, if the voter is anonymous
    #[must_use]
    pub fn voter_chat<T: Into<crate::types::Chat>>(mut self, val: T) -> Self {
        self.voter_chat = Some(Box::new(val.into()));
        self
    }

    /// The chat that changed the answer to the poll, if the voter is anonymous
    #[must_use]
    pub fn voter_chat_option<T: Into<crate::types::Chat>>(mut self, val: Option<T>) -> Self {
        self.voter_chat = val.map(|val| Box::new(val.into()));
        self
    }

    /// The user that changed the answer to the poll, if the voter isn't anonymous
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.user = Some(Box::new(val.into()));
        self
    }

    /// The user that changed the answer to the poll, if the voter isn't anonymous
    #[must_use]
    pub fn user_option<T: Into<crate::types::User>>(mut self, val: Option<T>) -> Self {
        self.user = val.map(|val| Box::new(val.into()));
        self
    }

    /// 0-based identifiers of chosen answer options. May be empty if the vote was retracted.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn option_ids<T: Into<Box<[i64]>>>(mut self, val: T) -> Self {
        self.option_ids = self
            .option_ids
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// 0-based identifiers of chosen answer options. May be empty if the vote was retracted.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn option_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.option_ids = self
            .option_ids
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// Persistent identifiers of the chosen answer options. May be empty if the vote was retracted.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn option_persistent_ids<T: Into<Box<[Box<str>]>>>(mut self, val: T) -> Self {
        self.option_persistent_ids = self
            .option_persistent_ids
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// Persistent identifiers of the chosen answer options. May be empty if the vote was retracted.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn option_persistent_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.option_persistent_ids = self
            .option_persistent_ids
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }
}
