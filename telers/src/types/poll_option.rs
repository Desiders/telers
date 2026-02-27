use serde::{Deserialize, Serialize};
/// This object contains information about one answer option in a poll.
/// # Documentation
/// <https://core.telegram.org/bots/api#polloption>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PollOption {
    /// Option text, 1-100 characters
    pub text: Box<str>,
    /// Special entities that appear in the option text. Currently, only custom emoji entities are allowed in poll option texts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// Number of users that voted for this option
    pub voter_count: i64,
}
impl PollOption {
    /// Creates a new `PollOption`.
    ///
    /// # Arguments
    /// * `text` - Option text, 1-100 characters
    /// * `voter_count` - Number of users that voted for this option
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<i64>>(text: T0, voter_count: T1) -> Self {
        Self {
            text: text.into(),
            text_entities: None,
            voter_count: voter_count.into(),
        }
    }

    /// Option text, 1-100 characters
    #[must_use]
    pub fn text<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.text = val.into();
        this
    }

    /// Special entities that appear in the option text. Currently, only custom emoji entities are allowed in poll option texts
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn text_entities<T: Into<Box<[crate::types::MessageEntity]>>>(self, val: T) -> Self {
        let mut this = self;
        this.text_entities = Some(
            this.text_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        this
    }

    /// Special entities that appear in the option text. Currently, only custom emoji entities are allowed in poll option texts
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn text_entity<T: Into<crate::types::MessageEntity>>(self, val: T) -> Self {
        let mut this = self;
        this.text_entities = Some(
            this.text_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// Special entities that appear in the option text. Currently, only custom emoji entities are allowed in poll option texts
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn text_entities_option<T: Into<Box<[crate::types::MessageEntity]>>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.text_entities = val.map(Into::into);
        this
    }

    /// Number of users that voted for this option
    #[must_use]
    pub fn voter_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.voter_count = val.into();
        this
    }
}
