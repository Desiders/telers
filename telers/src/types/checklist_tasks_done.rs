use crate::types::Message;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes a service message about checklist tasks marked as done or not done.
/// # Documentation
/// <https://core.telegram.org/bots/api#checklisttasksdone>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ChecklistTasksDone {
    /// Message containing the checklist whose tasks were marked as done or not done. Note that the [`Message`] object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    pub checklist_message: Option<Message>,
    /// Identifiers of the tasks that were marked as done
    pub marked_as_done_task_ids: Option<Box<[i64]>>,
    /// Identifiers of the tasks that were marked as not done
    pub marked_as_not_done_task_ids: Option<Box<[i64]>>,
}
