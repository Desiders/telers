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
    pub fn persistent_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.persistent_id = val.into();
        this
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

    /// Media added to the poll option
    #[must_use]
    pub fn media<T: Into<crate::types::PollMedia>>(self, val: T) -> Self {
        let mut this = self;
        this.media = Some(val.into());
        this
    }

    /// Media added to the poll option
    #[must_use]
    pub fn media_option<T: Into<crate::types::PollMedia>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.media = val.map(Into::into);
        this
    }

    /// Number of users who voted for this option; may be 0 if unknown
    #[must_use]
    pub fn voter_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.voter_count = val.into();
        this
    }

    /// User who added the option; omitted if the option wasn't added by a user after poll creation
    #[must_use]
    pub fn added_by_user<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.added_by_user = Some(Box::new(val.into()));
        this
    }

    /// User who added the option; omitted if the option wasn't added by a user after poll creation
    #[must_use]
    pub fn added_by_user_option<T: Into<crate::types::User>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.added_by_user = val.map(|val| Box::new(val.into()));
        this
    }

    /// Chat that added the option; omitted if the option wasn't added by a chat after poll creation
    #[must_use]
    pub fn added_by_chat<T: Into<crate::types::Chat>>(self, val: T) -> Self {
        let mut this = self;
        this.added_by_chat = Some(Box::new(val.into()));
        this
    }

    /// Chat that added the option; omitted if the option wasn't added by a chat after poll creation
    #[must_use]
    pub fn added_by_chat_option<T: Into<crate::types::Chat>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.added_by_chat = val.map(|val| Box::new(val.into()));
        this
    }

    /// Point in time (Unix timestamp) when the option was added; omitted if the option existed in the original poll
    #[must_use]
    pub fn addition_date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.addition_date = Some(val.into());
        this
    }

    /// Point in time (Unix timestamp) when the option was added; omitted if the option existed in the original poll
    #[must_use]
    pub fn addition_date_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.addition_date = val.map(Into::into);
        this
    }
}
