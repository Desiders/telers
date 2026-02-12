use crate::types::{Message, SuggestedPostPrice};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes a service message about the approval of a suggested post.
/// # Documentation
/// <https://core.telegram.org/bots/api#suggestedpostapproved>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SuggestedPostApproved {
    /// Message containing the suggested post. Note that the [`Message`] object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    pub checklist_message: Option<Message>,
    /// Amount paid for the post
    pub price: Option<SuggestedPostPrice>,
    /// Date when the post will be published
    pub send_date: Option<i64>,
}
