use serde::{Deserialize, Serialize};
/// This object contains information about one answer option in a poll.
/// # Documentation
/// <https://core.telegram.org/bots/api#polloption>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PollOption {
    /// Unique identifier of the option, persistent on option addition and deletion
    pub persistent_id: Box<str>,
    /// Option text, 1-100 characters
    pub text: Box<str>,
    /// Special entities that appear in the option text. Currently, only custom emoji entities are allowed in poll option texts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// Media added to the poll option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<crate::types::PollMedia>,
    /// Number of users who voted for this option; may be 0 if unknown
    pub voter_count: i64,
    /// User who added the option; omitted if the option wasn't added by a user after poll creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_by_user: Option<Box<crate::types::User>>,
    /// Chat that added the option; omitted if the option wasn't added by a chat after poll creation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_by_chat: Option<Box<crate::types::Chat>>,
    /// Point in time (Unix timestamp) when the option was added; omitted if the option existed in the original poll
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addition_date: Option<i64>,
}
impl PollOption {
    /// Creates a new `PollOption`.
    ///
    /// # Arguments
    /// * `persistent_id` - Unique identifier of the option, persistent on option addition and deletion
    /// * `text` - Option text, 1-100 characters
    /// * `voter_count` - Number of users who voted for this option; may be 0 if unknown
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>, T2: Into<i64>>(
        persistent_id: T0,
        text: T1,
        voter_count: T2,
    ) -> Self {
        Self {
            persistent_id: persistent_id.into(),
            text: text.into(),
            text_entities: None,
            media: None,
            voter_count: voter_count.into(),
            added_by_user: None,
            added_by_chat: None,
            addition_date: None,
        }
    }

    /// Unique identifier of the option, persistent on option addition and deletion
    #[must_use]
    pub fn persistent_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.persistent_id = val.into();
        self
    }

    /// Option text, 1-100 characters
    #[must_use]
    pub fn text<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.text = val.into();
        self
    }

    /// Special entities that appear in the option text. Currently, only custom emoji entities are allowed in poll option texts
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn text_entities<T: Into<Box<[crate::types::MessageEntity]>>>(mut self, val: T) -> Self {
        self.text_entities = Some(
            self.text_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// Special entities that appear in the option text. Currently, only custom emoji entities are allowed in poll option texts
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn text_entity<T: Into<crate::types::MessageEntity>>(mut self, val: T) -> Self {
        self.text_entities = Some(
            self.text_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// Special entities that appear in the option text. Currently, only custom emoji entities are allowed in poll option texts
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn text_entities_option<T: Into<Box<[crate::types::MessageEntity]>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.text_entities = val.map(Into::into);
        self
    }

    /// Media added to the poll option
    #[must_use]
    pub fn media<T: Into<crate::types::PollMedia>>(mut self, val: T) -> Self {
        self.media = Some(val.into());
        self
    }

    /// Media added to the poll option
    #[must_use]
    pub fn media_option<T: Into<crate::types::PollMedia>>(mut self, val: Option<T>) -> Self {
        self.media = val.map(Into::into);
        self
    }

    /// Number of users who voted for this option; may be 0 if unknown
    #[must_use]
    pub fn voter_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.voter_count = val.into();
        self
    }

    /// User who added the option; omitted if the option wasn't added by a user after poll creation
    #[must_use]
    pub fn added_by_user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.added_by_user = Some(Box::new(val.into()));
        self
    }

    /// User who added the option; omitted if the option wasn't added by a user after poll creation
    #[must_use]
    pub fn added_by_user_option<T: Into<crate::types::User>>(mut self, val: Option<T>) -> Self {
        self.added_by_user = val.map(|val| Box::new(val.into()));
        self
    }

    /// Chat that added the option; omitted if the option wasn't added by a chat after poll creation
    #[must_use]
    pub fn added_by_chat<T: Into<crate::types::Chat>>(mut self, val: T) -> Self {
        self.added_by_chat = Some(Box::new(val.into()));
        self
    }

    /// Chat that added the option; omitted if the option wasn't added by a chat after poll creation
    #[must_use]
    pub fn added_by_chat_option<T: Into<crate::types::Chat>>(mut self, val: Option<T>) -> Self {
        self.added_by_chat = val.map(|val| Box::new(val.into()));
        self
    }

    /// Point in time (Unix timestamp) when the option was added; omitted if the option existed in the original poll
    #[must_use]
    pub fn addition_date<T: Into<i64>>(mut self, val: T) -> Self {
        self.addition_date = Some(val.into());
        self
    }

    /// Point in time (Unix timestamp) when the option was added; omitted if the option existed in the original poll
    #[must_use]
    pub fn addition_date_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.addition_date = val.map(Into::into);
        self
    }
}
