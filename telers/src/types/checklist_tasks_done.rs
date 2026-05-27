use serde::{Deserialize, Serialize};
/// Describes a service message about checklist tasks marked as done or not done.
/// # Documentation
/// <https://core.telegram.org/bots/api#checklisttasksdone>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChecklistTasksDone {
    /// Message containing the checklist whose tasks were marked as done or not done. Note that the Message object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist_message: Option<Box<crate::types::Message>>,
    /// Identifiers of the tasks that were marked as done
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marked_as_done_task_ids: Option<Box<[i64]>>,
    /// Identifiers of the tasks that were marked as not done
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marked_as_not_done_task_ids: Option<Box<[i64]>>,
}
impl ChecklistTasksDone {
    /// Creates a new `ChecklistTasksDone`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            checklist_message: None,
            marked_as_done_task_ids: None,
            marked_as_not_done_task_ids: None,
        }
    }

    /// Message containing the checklist whose tasks were marked as done or not done. Note that the Message object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    #[must_use]
    pub fn checklist_message<T: Into<crate::types::Message>>(mut self, val: T) -> Self {
        self.checklist_message = Some(Box::new(val.into()));
        self
    }

    /// Message containing the checklist whose tasks were marked as done or not done. Note that the Message object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    #[must_use]
    pub fn checklist_message_option<T: Into<crate::types::Message>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.checklist_message = val.map(|val| Box::new(val.into()));
        self
    }

    /// Identifiers of the tasks that were marked as done
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn marked_as_done_task_ids<T: Into<Box<[i64]>>>(mut self, val: T) -> Self {
        self.marked_as_done_task_ids = Some(
            self.marked_as_done_task_ids
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// Identifiers of the tasks that were marked as done
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn marked_as_done_task_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.marked_as_done_task_ids = Some(
            self.marked_as_done_task_ids
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// Identifiers of the tasks that were marked as done
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn marked_as_done_task_ids_option<T: Into<Box<[i64]>>>(mut self, val: Option<T>) -> Self {
        self.marked_as_done_task_ids = val.map(Into::into);
        self
    }

    /// Identifiers of the tasks that were marked as not done
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn marked_as_not_done_task_ids<T: Into<Box<[i64]>>>(mut self, val: T) -> Self {
        self.marked_as_not_done_task_ids = Some(
            self.marked_as_not_done_task_ids
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// Identifiers of the tasks that were marked as not done
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn marked_as_not_done_task_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.marked_as_not_done_task_ids = Some(
            self.marked_as_not_done_task_ids
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// Identifiers of the tasks that were marked as not done
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn marked_as_not_done_task_ids_option<T: Into<Box<[i64]>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.marked_as_not_done_task_ids = val.map(Into::into);
        self
    }
}
impl Default for ChecklistTasksDone {
    fn default() -> Self {
        Self::new()
    }
}
