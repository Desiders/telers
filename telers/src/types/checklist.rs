use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::types::{ChecklistTask, MessageEntity};

/// Describes a checklist.
/// # Documentation
/// <https://core.telegram.org/bots/api#checklist>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Checklist {
    /// Title of the checklist
    pub title: Box<str>,
    /// Special entities that appear in the checklist title
    pub title_entities: Option<Box<[MessageEntity]>>,
    /// List of tasks in the checklist
    pub tasks: Box<[ChecklistTask]>,
    /// `true`, if users other than the creator of the list can add tasks to the list
    pub others_can_add_tasks: Option<bool>,
    /// `true`, if users other than the creator of the list can mark tasks as done or not done
    pub others_can_mark_tasks_as_done: Option<bool>,
}
