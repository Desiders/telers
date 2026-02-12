use crate::types::{ChecklistTask, Message};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes a service message about tasks added to a checklist.
/// # Documentation
/// <https://core.telegram.org/bots/api#checklisttasksadded>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ChecklistTasksAdded {
    /// Message containing the checklist to which the tasks were added. Note that the [`Message`] object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    pub checklist_message: Option<Message>,
    /// List of tasks added to the checklist
    pub tasks: Box<[ChecklistTask]>,
}
