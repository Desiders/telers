use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::types::{Chat, MessageEntity, User};

/// Describes a task in a checklist.
/// # Documentation
/// <https://core.telegram.org/bots/api#checklisttask>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ChecklistTask {
    /// Unique identifier of the task
    pub id: i64,
    /// Text of the task
    pub text: Box<str>,
    /// Special entities that appear in the task text
    pub text_entities: Option<Box<[MessageEntity]>>,
    /// User that completed the task; omitted if the task wasn't completed by a user
    pub completed_by_user: Option<User>,
    /// Chat that completed the task; omitted if the task wasn't completed by a chat
    pub completed_by_chat: Option<Chat>,
    /// Point in time (Unix timestamp) when the task was completed; 0 if the task wasn't completed
    pub completion_date: Option<i64>,
}
