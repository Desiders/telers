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
    /// List of special entities that appear in the title, which can be specified instead of `parse_mode`. Currently, only bold, italic, underline, strikethrough, spoiler, `custom_emoji`, and `date_time` entities are allowed.
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
    pub fn title<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.title = val.into();
        self
    }

    /// Mode for parsing entities in the title. See formatting options for more details.
    #[must_use]
    pub fn parse_mode<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.parse_mode = Some(val.into());
        self
    }

    /// Mode for parsing entities in the title. See formatting options for more details.
    #[must_use]
    pub fn parse_mode_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.parse_mode = val.map(Into::into);
        self
    }

    /// List of special entities that appear in the title, which can be specified instead of `parse_mode`. Currently, only bold, italic, underline, strikethrough, spoiler, `custom_emoji`, and `date_time` entities are allowed.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn title_entities<T: Into<Box<[crate::types::MessageEntity]>>>(mut self, val: T) -> Self {
        self.title_entities = Some(
            self.title_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// List of special entities that appear in the title, which can be specified instead of `parse_mode`. Currently, only bold, italic, underline, strikethrough, spoiler, `custom_emoji`, and `date_time` entities are allowed.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn title_entity<T: Into<crate::types::MessageEntity>>(mut self, val: T) -> Self {
        self.title_entities = Some(
            self.title_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// List of special entities that appear in the title, which can be specified instead of `parse_mode`. Currently, only bold, italic, underline, strikethrough, spoiler, `custom_emoji`, and `date_time` entities are allowed.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn title_entities_option<T: Into<Box<[crate::types::MessageEntity]>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.title_entities = val.map(Into::into);
        self
    }

    /// List of 1-30 tasks in the checklist
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn tasks<T: Into<Box<[crate::types::InputChecklistTask]>>>(mut self, val: T) -> Self {
        self.tasks = self
            .tasks
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// List of 1-30 tasks in the checklist
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn task<T: Into<crate::types::InputChecklistTask>>(mut self, val: T) -> Self {
        self.tasks = self
            .tasks
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// Pass `true` if other users can add tasks to the checklist
    #[must_use]
    pub fn others_can_add_tasks<T: Into<bool>>(mut self, val: T) -> Self {
        self.others_can_add_tasks = Some(val.into());
        self
    }

    /// Pass `true` if other users can add tasks to the checklist
    #[must_use]
    pub fn others_can_add_tasks_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.others_can_add_tasks = val.map(Into::into);
        self
    }

    /// Pass `true` if other users can mark tasks as done or not done in the checklist
    #[must_use]
    pub fn others_can_mark_tasks_as_done<T: Into<bool>>(mut self, val: T) -> Self {
        self.others_can_mark_tasks_as_done = Some(val.into());
        self
    }

    /// Pass `true` if other users can mark tasks as done or not done in the checklist
    #[must_use]
    pub fn others_can_mark_tasks_as_done_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.others_can_mark_tasks_as_done = val.map(Into::into);
        self
    }
}
