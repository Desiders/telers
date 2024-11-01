use super::Message;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object represents a service message about the completion of a giveaway without public winners.
/// # Documentation
/// <https://core.telegram.org/bots/api#giveawaycompleted>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GiveawayCompleted {
    /// Number of winners in the giveaway
    pub winner_count: i64,
    /// Number of undistributed prizes
    pub unclaimed_prize_count: Option<i64>,
    /// Message with the giveaway that was completed, if it wasn't deleted
    pub giveaway_message: Option<Message>,
    /// `true`, if the giveaway is a Telegram Star giveaway. Otherwise, currently, the giveaway is a Telegram Premium giveaway.
    pub is_star_giveaway: Option<bool>,
}
