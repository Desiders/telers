use serde::Deserialize;

use super::{UniqueGift, User};

/// Describes a unique gift received and owned by a user or a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#ownedgiftunique>
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct OwnedGiftUnique {
    /// Information about the unique gift
    pub gift: UniqueGift,
    /// Unique identifier of the received gift for the bot; for gifts received on behalf of business accounts only
    pub owned_gift_id: Option<Box<str>>,
    /// Sender of the gift if it is a known user
    pub sender_user: Option<User>,
    /// Date the gift was sent in Unix time
    pub send_date: i64,
    /// `true`, if the gift is displayed on the account's profile page; for gifts received on behalf of business accounts only
    pub is_saved: Option<bool>,
    /// `true`, if the gift can be transferred to another owner; for gifts received on behalf of business accounts only
    pub can_be_transferred: Option<bool>,
    /// Number of Telegram Stars that must be paid to transfer the gift; omitted if the bot cannot transfer the gift
    pub transfer_star_count: Option<i64>,
    /// Point in time (Unix timestamp) when the gift can be transferred. If it is in the past, then the gift can be transferred now
    pub next_transfer_date: Option<i64>,
}
