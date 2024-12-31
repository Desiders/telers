use super::{Chat, User};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Contains information about the affiliate that received a commission via this transaction
/// # Documentation
/// <https://core.telegram.org/bots/api#affiliateinfo>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AffiliateInfo {
    /// The bot or the user that received an affiliate commission if it was received by a bot or a user
    pub affiliate_user: Option<User>,
    /// The chat that received an affiliate commission if it was received by a chat
    pub affiliate_chat: Option<Chat>,
    /// The number of Telegram Stars received by the affiliate for each 1000 Telegram Stars received by the bot from referred users
    pub commission_per_mille: i64,
    /// Integer amount of Telegram Stars received by the affiliate from the transaction, rounded to 0; can be negative for refunds
    pub amount: i64,
    /// The number of 1/1000000000 shares of Telegram Stars received by the affiliate; from -999999999 to 999999999; can be negative for refunds
    pub nanostar_amount: Option<i64>,
}
