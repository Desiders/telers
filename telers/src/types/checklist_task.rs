use serde::{Deserialize, Serialize};
/// Describes a task in a checklist.
/// # Documentation
/// <https://core.telegram.org/bots/api#checklisttask>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChecklistTask {
    /// Unique identifier of the task
    pub id: i64,
    /// Text of the task
    pub text: Box<str>,
    /// Special entities that appear in the task text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// User that completed the task; omitted if the task wasn't completed by a user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_by_user: Option<Box<crate::types::User>>,
    /// Chat that completed the task; omitted if the task wasn't completed by a chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_by_chat: Option<Box<crate::types::Chat>>,
    /// Point in time (Unix timestamp) when the task was completed; 0 if the task wasn't completed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<i64>,
}
impl ChecklistTask {
    /// Creates a new `ChecklistTask`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier of the task
    /// * `text` - Text of the task
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<Box<str>>>(id: T0, text: T1) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            text_entities: None,
            completed_by_user: None,
            completed_by_chat: None,
            completion_date: None,
        }
    }

    /// Unique identifier of the task
    #[must_use]
    pub fn id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.id = val.into();
        this
    }

    /// Text of the task
    #[must_use]
    pub fn text<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.text = val.into();
        this
    }

    /// Special entities that appear in the task text
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

    /// Special entities that appear in the task text
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

    /// Special entities that appear in the task text
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

    /// User that completed the task; omitted if the task wasn't completed by a user
    #[must_use]
    pub fn completed_by_user<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.completed_by_user = Some(Box::new(val.into()));
        this
    }

    /// User that completed the task; omitted if the task wasn't completed by a user
    #[must_use]
    pub fn completed_by_user_option<T: Into<crate::types::User>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.completed_by_user = val.map(|val| Box::new(val.into()));
        this
    }

    /// Chat that completed the task; omitted if the task wasn't completed by a chat
    #[must_use]
    pub fn completed_by_chat<T: Into<crate::types::Chat>>(self, val: T) -> Self {
        let mut this = self;
        this.completed_by_chat = Some(Box::new(val.into()));
        this
    }

    /// Chat that completed the task; omitted if the task wasn't completed by a chat
    #[must_use]
    pub fn completed_by_chat_option<T: Into<crate::types::Chat>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.completed_by_chat = val.map(|val| Box::new(val.into()));
        this
    }

    /// Point in time (Unix timestamp) when the task was completed; 0 if the task wasn't completed
    #[must_use]
    pub fn completion_date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.completion_date = Some(val.into());
        this
    }

    /// Point in time (Unix timestamp) when the task was completed; 0 if the task wasn't completed
    #[must_use]
    pub fn completion_date_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.completion_date = val.map(Into::into);
        this
    }
}
