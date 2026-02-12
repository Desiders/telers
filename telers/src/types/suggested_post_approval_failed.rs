use crate::types::{Message, SuggestedPostPrice};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes a service message about the failed approval of a suggested post. Currently, only caused by insufficient user funds at the time of approval.
/// # Documentation
/// <https://core.telegram.org/bots/api#suggestedpostapprovalfailed>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SuggestedPostApprovalFailed {
    /// Message containing the suggested post whose approval has failed. Note that the [`Message`] object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    pub suggested_post_message: Option<Message>,
    /// Expected price of the post
    pub price: SuggestedPostPrice,
}
