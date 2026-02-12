use super::Sticker;

use crate::types::Chat;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object represents a gift that can be sent by the bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#gift>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Gift {
    /// Unique identifier of the gift
    pub id: Box<str>,
    /// The sticker that represents the gift
    pub sticker: Box<Sticker>,
    /// The number of Telegram Stars that must be paid to send the sticker
    pub star_count: i64,
    /// The number of Telegram Stars that must be paid to upgrade the gift to a unique one
    pub upgrade_star_count: Option<i64>,
    /// `true`, if the gift can only be purchased by Telegram Premium subscribers
    pub is_premium: Option<bool>,
    /// `true`, if the gift can be used (after being upgraded) to customize a user's appearance
    pub has_colors: Option<bool>,
    /// The total number of gifts of this type that can be sent by all users; for limited gifts only
    pub total_count: Option<i64>,
    /// The number of remaining gifts of this type that can be sent by all users; for limited gifts only
    pub remaining_count: Option<i64>,
    /// The total number of gifts of this type that can be sent by the bot; for limited gifts only
    pub personal_total_count: Option<i64>,
    /// The number of remaining gifts of this type that can be sent by the bot; for limited gifts only
    pub personal_remaining_count: Option<i64>,
    /// The total number of different unique gifts that can be obtained by upgrading the gift
    pub unique_gift_variant_count: Option<i64>,
    /// Information about the chat that published the gift
    pub publisher_chat: Option<Chat>,
}
