use crate::types::Message;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes a service message about a successful payment for a suggested post.
/// # Documentation
/// <https://core.telegram.org/bots/api#suggestedpostpaid>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SuggestedPostPaid {
    /// Message containing the suggested post. Note that the [`Message`] object in this field will not contain the `reply_to_message` field even if it itself is a reply.
    pub suggested_post_message: Option<Message>,
    /// Currency in which the payment was made. Currently, one of “XTR” for Telegram Stars or “TON” for toncoins
    pub currency: Option<Box<str>>,
    /// The amount of the currency that was received by the channel in nanotoncoins; for payments in toncoins only
    pub amount: Option<i64>,
    /// The amount of Telegram Stars that was received by the channel; for payments in Telegram Stars only
    pub star_amount: Option<i64>,
}
