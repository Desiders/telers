use serde::{Deserialize, Serialize};
/// Describes a service message about an option deleted from a poll.
/// # Documentation
/// <https://core.telegram.org/bots/api#polloptiondeleted>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PollOptionDeleted {
    /// Message containing the poll from which the option was deleted, if known. Note that the Message object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_message: Option<Box<crate::types::MaybeInaccessibleMessage>>,
    /// Unique identifier of the deleted option
    pub option_persistent_id: Box<str>,
    /// Option text
    pub option_text: Box<str>,
    /// Special entities that appear in the `option_text`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_text_entities: Option<Box<[crate::types::MessageEntity]>>,
}
impl PollOptionDeleted {
    /// Creates a new `PollOptionDeleted`.
    ///
    /// # Arguments
    /// * `option_persistent_id` - Unique identifier of the deleted option
    /// * `option_text` - Option text
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(
        option_persistent_id: T0,
        option_text: T1,
    ) -> Self {
        Self {
            poll_message: None,
            option_persistent_id: option_persistent_id.into(),
            option_text: option_text.into(),
            option_text_entities: None,
        }
    }

    /// Message containing the poll from which the option was deleted, if known. Note that the Message object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    #[must_use]
    pub fn poll_message<T: Into<crate::types::MaybeInaccessibleMessage>>(self, val: T) -> Self {
        let mut this = self;
        this.poll_message = Some(Box::new(val.into()));
        this
    }

    /// Message containing the poll from which the option was deleted, if known. Note that the Message object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    #[must_use]
    pub fn poll_message_option<T: Into<crate::types::MaybeInaccessibleMessage>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.poll_message = val.map(|val| Box::new(val.into()));
        this
    }

    /// Unique identifier of the deleted option
    #[must_use]
    pub fn option_persistent_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.option_persistent_id = val.into();
        this
    }

    /// Option text
    #[must_use]
    pub fn option_text<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.option_text = val.into();
        this
    }

    /// Special entities that appear in the `option_text`
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn option_text_entities<T: Into<Box<[crate::types::MessageEntity]>>>(self, val: T) -> Self {
        let mut this = self;
        this.option_text_entities = Some(
            this.option_text_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        this
    }

    /// Special entities that appear in the `option_text`
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn option_text_entity<T: Into<crate::types::MessageEntity>>(self, val: T) -> Self {
        let mut this = self;
        this.option_text_entities = Some(
            this.option_text_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// Special entities that appear in the `option_text`
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn option_text_entities_option<T: Into<Box<[crate::types::MessageEntity]>>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.option_text_entities = val.map(Into::into);
        this
    }
}
