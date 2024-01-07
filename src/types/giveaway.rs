use serde::Deserialize;

use super::Chat;

/// This object represents a message about a scheduled giveaway.
/// # Documentation
/// <https://core.telegram.org/bots/api#giveaway>
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct Giveaway {
    /// The list of chats which the user must join to participate in the giveaway
    pub chats: Box<[Chat]>,
    /// Point in time (Unix timestamp) when winners of the giveaway will be selected
    pub winners_selection_date: i64,
    /// The number of users which are supposed to be selected as winners of the giveaway
    pub winner_count: i64,
    /// `true`, if only users who join the chats after the giveaway started should be eligible to win
    pub only_new_members: Option<bool>,
    /// `true`, if the list of giveaway winners will be visible to everyone
    pub has_public_winners: Option<bool>,
    /// Description of additional giveaway prize
    pub prize_description: Option<Box<str>>,
    /// A list of two-letter [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country codes indicating the countries from which eligible users for the giveaway must come. If empty, then all users can participate in the giveaway. Users with a phone number that was bought on Fragment can always participate in giveaways.
    pub country_codes: Option<Box<[Box<str>]>>,
    /// The number of months the Telegram Premium subscription won from the giveaway will be active for
    pub premium_subscription_month_count: Option<i64>,
}
