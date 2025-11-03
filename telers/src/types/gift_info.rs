use crate::types::MessageEntity;

use super::Gift;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes a service message about a regular gift that was sent or received.
/// # Documentation
/// <https://core.telegram.org/bots/api#giftinfo>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GiftInfo {
    /// Information about the gift
    pub gift: Gift,
    /// Unique identifier of the received gift for the bot; only present for gifts received on behalf of business accounts
    pub owned_gift_id: Option<Box<str>>,
    /// Number of Telegram Stars that can be claimed by the receiver by converting the gift; omitted if conversion to Telegram Stars is impossible
    pub convert_star_count: Option<i64>,
    /// Number of Telegram Stars that were prepaid by the sender for the ability to upgrade the gift
    pub prepaid_upgrade_star_count: Option<i64>,
    /// `true`, if the gift can be upgraded to a unique gift
    pub can_be_upgraded: Option<bool>,
    /// Text of the message that was added to the gift
    pub text: Option<Box<str>>,
    /// Special entities that appear in the text
    pub entities: Option<Box<[MessageEntity]>>,
    /// `true`, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    pub is_private: Option<bool>,
}
