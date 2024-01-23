use super::Message;

use serde::Deserialize;

/// This object represents a service message about the completion of a giveaway without public winners.
/// # Documentation
/// <https://core.telegram.org/bots/api#giveawaycompleted>
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct GiveawayCompleted {
    /// Number of winners in the giveaway
    pub winner_count: i64,
    /// Number of undistributed prizes
    pub unclaimed_prize_count: Option<i64>,
    /// Message with the giveaway that was completed, if it wasn't deleted
    pub giveaway_message: Option<Box<Message>>,
}
