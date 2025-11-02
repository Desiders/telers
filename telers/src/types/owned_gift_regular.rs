use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{Gift, MessageEntity, User};

/// Describes a regular gift owned by a user or a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#ownedgiftregular>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OwnedGiftRegular {
    /// Information about the regular gift
    pub gift: Gift,
    /// Unique identifier of the gift for the bot; for gifts received on behalf of business accounts only
    pub owned_gift_id: Option<Box<str>>,
    /// Sender of the gift if it is a known user
    pub sender_user: Option<User>,
    /// Date the gift was sent in Unix time
    pub send_date: i64,
    /// Text of the message that was added to the gift
    pub text: Option<Box<str>>,
    /// Special entities that appear in the text
    pub entities: Option<Box<[MessageEntity]>>,
    /// `true`, if the sender and gift text are shown only to the gift receiver; otherwise, everyone will be able to see them
    pub is_private: Option<bool>,
    /// `true`, if the gift is displayed on the account's profile page; for gifts received on behalf of business accounts only
    pub is_saved: Option<bool>,
    /// `true`, if the gift can be upgraded to a unique gift; for gifts received on behalf of business accounts only
    pub can_be_upgraded: Option<bool>,
    /// `true`, if the gift was refunded and isn't available anymore
    pub was_refunded: Option<bool>,
    /// Number of Telegram Stars that can be claimed by the receiver instead of the gift; omitted if the gift cannot be converted to Telegram Stars
    pub convert_star_count: Option<i64>,
    /// Number of Telegram Stars that were paid by the sender for the ability to upgrade the gift
    pub prepaid_upgrade_star_count: Option<i64>,
}
