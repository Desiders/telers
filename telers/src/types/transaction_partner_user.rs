use super::{Gift, PaidMedia, User};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes a transaction with a user.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartneruser>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TransactionPartnerUser {
    /// Information about the user
    pub user: User,
    /// Bot-specified invoice payload
    pub invoice_payload: Option<Box<str>>,
    /// The duration of the paid subscription
    pub subscription_period: Option<i64>,
    /// Information about the paid media bought by the user
    pub paid_media: Option<Box<[PaidMedia]>>,
    /// Bot-specified paid media payload
    pub paid_media_payload: Option<Box<str>>,
    /// The gift sent to the user by the bot
    pub gift: Option<Gift>,
}
