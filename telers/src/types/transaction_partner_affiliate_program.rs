use super::User;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes the affiliate program that issued the affiliate commission received via this transaction.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartneraffiliateprogram>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TransactionPartnerAffiliateProgram {
    /// Information about the bot that sponsored the affiliate program
    pub sponsor_user: Option<User>,
    /// The number of Telegram Stars received by the bot for each 1000 Telegram Stars received by the affiliate program sponsor from referred users
    pub commission_per_mille: i64,
}
