use crate::types::UniqueGift;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object describes a unique gift that was upgraded from a regular gift.
/// # Documentation
/// <https://core.telegram.org/bots/api#uniquegift>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UniqueGiftInfo {
    /// Information about the gift
    pub gift: UniqueGift,
    /// Origin of the gift. Currently, either “upgrade” for gifts upgraded from regular gifts, “transfer” for gifts transferred from other users or channels, “resale” for gifts bought from other users, “gifted_upgrade” for upgrades purchased after the gift was sent, or “offer” for gifts bought or sold through gift purchase offers
    pub origin: Box<str>,
    /// For gifts bought from other users, the currency in which the payment for the gift was done. Currently, one of “XTR” for Telegram Stars or “TON” for toncoins.
    pub last_resale_currency: Option<Box<str>>,
    /// For gifts bought from other users, the price paid for the gift in either Telegram Stars or nanotoncoins
    pub last_resale_amount: Option<i64>,
    /// Unique identifier of the received gift for the bot; only present for gifts received on behalf of business accounts
    pub owned_gift_id: Option<Box<str>>,
    /// Number of Telegram Stars that must be paid to transfer the gift; omitted if the bot cannot transfer the gift
    pub transfer_star_count: Option<i64>,
    /// Point in time (Unix timestamp) when the gift can be transferred. If it is in the past, then the gift can be transferred now
    pub next_transfer_date: Option<i64>,
}
