use serde::{Deserialize, Serialize};

/// DDescribes a service message about a change in the price of direct messages sent to a channel chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#directmessagepricechanged>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DirectMessagePriceChanged {
    /// `true`, if direct messages are enabled for the channel chat; false otherwise
    pub are_direct_messages_enabled: bool,
    /// The new number of Telegram Stars that must be paid by users for each direct message sent to the channel. Does not apply to users who have been exempted by administrators. Defaults to 0.
    pub direct_message_star_count: Option<i64>,
}
