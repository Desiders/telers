use super::User;

use serde::Deserialize;

/// The boost was obtained by the creation of a Telegram Premium giveaway. This boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboostsourcegiveaway>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct ChatBoostSourceGiveaway {
    /// Identifier of a message in the chat with the giveaway; the message could have been deleted already. May be 0 if the message isn't sent yet.
    pub giveaway_message_id: i64,
    /// User that won the prize in the giveaway if any
    pub user: Option<User>,
    /// `true`, if the giveaway was completed, but there was no user to win the prize
    pub is_unclaimed: Option<bool>,
}
