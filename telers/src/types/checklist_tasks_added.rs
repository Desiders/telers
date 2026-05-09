use serde::{Deserialize, Serialize};
/// Describes a service message about tasks added to a checklist.
/// # Documentation
/// <https://core.telegram.org/bots/api#checklisttasksadded>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChecklistTasksAdded {
    /// Message containing the checklist to which the tasks were added. Note that the Message object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist_message: Option<Box<crate::types::Message>>,
    /// List of tasks added to the checklist
    pub tasks: Box<[crate::types::ChecklistTask]>,
}
impl ChecklistTasksAdded {
    /// Creates a new `ChecklistTasksAdded`.
    ///
    /// # Arguments
    /// * `tasks` - List of tasks added to the checklist
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0Item: Into<crate::types::ChecklistTask>, T0: IntoIterator<Item = T0Item>>(
        tasks: T0,
    ) -> Self {
        Self {
            checklist_message: None,
            tasks: tasks.into_iter().map(Into::into).collect(),
        }
    }

    /// Message containing the checklist to which the tasks were added. Note that the Message object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    #[must_use]
    pub fn checklist_message<T: Into<crate::types::Message>>(mut self, val: T) -> Self {
        self.checklist_message = Some(Box::new(val.into()));
        self
    }

    /// Message containing the checklist to which the tasks were added. Note that the Message object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    #[must_use]
    pub fn checklist_message_option<T: Into<crate::types::Message>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.checklist_message = val.map(|val| Box::new(val.into()));
        self
    }

    /// List of tasks added to the checklist
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

    /// List of tasks added to the checklist
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
}
