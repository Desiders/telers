use crate::types::Message;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes a service message about the rejection of a suggested post.
/// # Documentation
/// <https://core.telegram.org/bots/api#suggestedpostdeclined>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SuggestedPostDeclined {
    /// Message containing the suggested post. Note that the [`Message`] object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    pub suggested_post_message: Option<Message>,
    /// Comment with which the post was declined
    pub comment: Option<Box<str>>,
}
