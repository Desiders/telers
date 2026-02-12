use crate::types::Message;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes a service message about a payment refund for a suggested post.
/// # Documentation
/// <https://core.telegram.org/bots/api#suggestedpostrefunded>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SuggestedPostRefunded {
    /// Message containing the suggested post. Note that the [`Message`] object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    pub suggested_post_message: Option<Message>,
    /// Reason for the refund. Currently, one of “post_deleted” if the post was deleted within 24 hours of being posted or removed from scheduled messages without being posted, or “payment_refunded” if the payer refunded their payment.
    pub reason: Box<str>,
}
