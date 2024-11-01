use super::User;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// The boost was obtained by the creation of a Telegram Premium giveaway. This boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboostsourcegiveaway>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ChatBoostSourceGiveaway {
    /// Identifier of a message in the chat with the giveaway; the message could have been deleted already. May be 0 if the message isn't sent yet.
    pub giveaway_message_id: i64,
    /// User that won the prize in the giveaway if any
    pub user: Option<User>,
    /// The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
    pub prize_star_count: i64,
    /// `true`, if the giveaway was completed, but there was no user to win the prize
    pub is_unclaimed: Option<bool>,
}
