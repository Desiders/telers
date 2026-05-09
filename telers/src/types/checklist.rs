use serde::{Deserialize, Serialize};
/// Describes a checklist.
/// # Documentation
/// <https://core.telegram.org/bots/api#checklist>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Checklist {
    /// Title of the checklist
    pub title: Box<str>,
    /// Special entities that appear in the checklist title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// List of tasks in the checklist
    pub tasks: Box<[crate::types::ChecklistTask]>,
    /// `true`, if users other than the creator of the list can add tasks to the list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub others_can_add_tasks: Option<bool>,
    /// `true`, if users other than the creator of the list can mark tasks as done or not done
    #[serde(skip_serializing_if = "Option::is_none")]
    pub others_can_mark_tasks_as_done: Option<bool>,
}
impl Checklist {
    /// Creates a new `Checklist`.
    ///
    /// # Arguments
    /// * `title` - Title of the checklist
    /// * `tasks` - List of tasks in the checklist
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1Item: Into<crate::types::ChecklistTask>,
        T1: IntoIterator<Item = T1Item>,
    >(
        title: T0,
        tasks: T1,
    ) -> Self {
        Self {
            title: title.into(),
            title_entities: None,
            tasks: tasks.into_iter().map(Into::into).collect(),
            others_can_add_tasks: None,
            others_can_mark_tasks_as_done: None,
        }
    }

    /// Title of the checklist
    #[must_use]
    pub fn title<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.title = val.into();
        self
    }

    /// Special entities that appear in the checklist title
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

    /// Special entities that appear in the checklist title
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

    /// Special entities that appear in the checklist title
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

    /// List of tasks in the checklist
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn tasks<T: Into<Box<[crate::types::ChecklistTask]>>>(mut self, val: T) -> Self {
        self.tasks = self
            .tasks
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// List of tasks in the checklist
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn task<T: Into<crate::types::ChecklistTask>>(mut self, val: T) -> Self {
        self.tasks = self
            .tasks
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// `true`, if users other than the creator of the list can add tasks to the list
    #[must_use]
    pub fn others_can_add_tasks<T: Into<bool>>(mut self, val: T) -> Self {
        self.others_can_add_tasks = Some(val.into());
        self
    }

    /// `true`, if users other than the creator of the list can add tasks to the list
    #[must_use]
    pub fn others_can_add_tasks_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.others_can_add_tasks = val.map(Into::into);
        self
    }

    /// `true`, if users other than the creator of the list can mark tasks as done or not done
    #[must_use]
    pub fn others_can_mark_tasks_as_done<T: Into<bool>>(mut self, val: T) -> Self {
        self.others_can_mark_tasks_as_done = Some(val.into());
        self
    }

    /// `true`, if users other than the creator of the list can mark tasks as done or not done
    #[must_use]
    pub fn others_can_mark_tasks_as_done_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.others_can_mark_tasks_as_done = val.map(Into::into);
        self
    }
}
