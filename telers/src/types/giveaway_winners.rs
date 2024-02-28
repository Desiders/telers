use serde::Deserialize;

use super::{Chat, User};

/// This object represents a message about the completion of a giveaway with public winners.
/// # Documentation
/// <https://core.telegram.org/bots/api#giveawaywinners>
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct GiveawayWinners {
    /// The chat that created the giveaway
    pub chat: Chat,
    /// Identifier of the message with the giveaway in the chat
    pub giveaway_message_id: i64,
    /// Point in time (Unix timestamp) when winners of the giveaway were selected
    pub winners_selection_date: i64,
    /// Total number of winners in the giveaway
    pub winner_count: i64,
    /// List of up to 100 winners of the giveaway
    pub winners: Box<[User]>,
    /// The number of other chats the user had to join in order to be eligible for the giveaway
    pub additional_chat_count: Option<i64>,
    /// The number of months the Telegram Premium subscription won from the giveaway will be active for
    pub premium_subscription_month_count: Option<i64>,
    /// Number of undistributed prizes
    pub unclaimed_prize_count: Option<i64>,
    /// `true`, if only users who had joined the chats after the giveaway started were eligible to win
    pub only_new_members: Option<bool>,
    /// `true`, if the giveaway was canceled because the payment for it was refunded
    pub was_refunded: Option<bool>,
    /// Description of additional giveaway prize
    pub additional_prize_description: Option<Box<str>>,
}
