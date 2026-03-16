use serde::{Deserialize, Serialize};
/// Describes a checklist to create.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputchecklist>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputChecklist {
    /// Title of the checklist; 1-255 characters after entities parsing
    pub title: Box<str>,
    /// Mode for parsing entities in the title. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<Box<str>>,
    /// List of special entities that appear in the title, which can be specified instead of `parse_mode`. Currently, only bold, italic, underline, strikethrough, spoiler, and `custom_emoji` entities are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// List of 1-30 tasks in the checklist
    pub tasks: Box<[crate::types::InputChecklistTask]>,
    /// Pass `true` if other users can add tasks to the checklist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub others_can_add_tasks: Option<bool>,
    /// Pass `true` if other users can mark tasks as done or not done in the checklist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub others_can_mark_tasks_as_done: Option<bool>,
}
impl InputChecklist {
    /// Creates a new `InputChecklist`.
    ///
    /// # Arguments
    /// * `title` - Title of the checklist; 1-255 characters after entities parsing
    /// * `tasks` - List of 1-30 tasks in the checklist
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1Item: Into<crate::types::InputChecklistTask>,
        T1: IntoIterator<Item = T1Item>,
    >(
        title: T0,
        tasks: T1,
    ) -> Self {
        Self {
            title: title.into(),
            parse_mode: None,
            title_entities: None,
            tasks: tasks.into_iter().map(Into::into).collect(),
            others_can_add_tasks: None,
            others_can_mark_tasks_as_done: None,
        }
    }

    /// Title of the checklist; 1-255 characters after entities parsing
    #[must_use]
    pub fn title<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.title = val.into();
        this
    }

    /// Mode for parsing entities in the title. See formatting options for more details.
    #[must_use]
    pub fn parse_mode<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.parse_mode = Some(val.into());
        this
    }

    /// Mode for parsing entities in the title. See formatting options for more details.
    #[must_use]
    pub fn parse_mode_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.parse_mode = val.map(Into::into);
        this
    }

    /// List of special entities that appear in the title, which can be specified instead of `parse_mode`. Currently, only bold, italic, underline, strikethrough, spoiler, and `custom_emoji` entities are allowed.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn title_entities<T: Into<Box<[crate::types::MessageEntity]>>>(self, val: T) -> Self {
        let mut this = self;
        this.title_entities = Some(
            this.title_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        this
    }

    /// List of special entities that appear in the title, which can be specified instead of `parse_mode`. Currently, only bold, italic, underline, strikethrough, spoiler, and `custom_emoji` entities are allowed.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn title_entity<T: Into<crate::types::MessageEntity>>(self, val: T) -> Self {
        let mut this = self;
        this.title_entities = Some(
            this.title_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// List of special entities that appear in the title, which can be specified instead of `parse_mode`. Currently, only bold, italic, underline, strikethrough, spoiler, and `custom_emoji` entities are allowed.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn title_entities_option<T: Into<Box<[crate::types::MessageEntity]>>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.title_entities = val.map(Into::into);
        this
    }

    /// List of 1-30 tasks in the checklist
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn tasks<T: Into<Box<[crate::types::InputChecklistTask]>>>(self, val: T) -> Self {
        let mut this = self;
        this.tasks = this
            .tasks
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        this
    }

    /// List of 1-30 tasks in the checklist
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn task<T: Into<crate::types::InputChecklistTask>>(self, val: T) -> Self {
        let mut this = self;
        this.tasks = this
            .tasks
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }

    /// Pass `true` if other users can add tasks to the checklist
    #[must_use]
    pub fn others_can_add_tasks<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.others_can_add_tasks = Some(val.into());
        this
    }

    /// Pass `true` if other users can add tasks to the checklist
    #[must_use]
    pub fn others_can_add_tasks_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.others_can_add_tasks = val.map(Into::into);
        this
    }

    /// Pass `true` if other users can mark tasks as done or not done in the checklist
    #[must_use]
    pub fn others_can_mark_tasks_as_done<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.others_can_mark_tasks_as_done = Some(val.into());
        this
    }

    /// Pass `true` if other users can mark tasks as done or not done in the checklist
    #[must_use]
    pub fn others_can_mark_tasks_as_done_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.others_can_mark_tasks_as_done = val.map(Into::into);
        this
    }
}
