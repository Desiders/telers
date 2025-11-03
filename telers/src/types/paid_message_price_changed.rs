use serde::{Deserialize, Serialize};

/// Describes a service message about a change in the price of paid messages within a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#paidmessagepricechanged>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PaidMessagePriceChanged {
    /// The new number of Telegram Stars that must be paid by non-administrator users of the supergroup chat for each sent message
    pub paid_message_star_count: i64,
}
