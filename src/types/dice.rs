use serde::Deserialize;
use std::ops::{Range, RangeInclusive};

/// This object represents an animated emoji that displays a random value.
/// <https://core.telegram.org/bots/api#dice>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct Dice {
    /// Emoji on which the dice throw animation is based
    pub emoji: String,
    /// Value of the dice, 1-6 for '🎲', '🎯' and '🎳' base emoji, 1-5 for '🏀' and '⚽' base emoji, 1-64 for '🎰' base emoji
    pub value: i64,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Emoji {
    Dice,
    Dart,
    Basketball,
    Football,
    SlotMachine,
}

impl Emoji {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Emoji::Dice => "🎲",
            Emoji::Dart => "🎯",
            Emoji::Basketball => "🏀",
            Emoji::Football => "⚽",
            Emoji::SlotMachine => "🎰",
        }
    }
}

impl From<Emoji> for RangeInclusive<i64> {
    fn from(val: Emoji) -> Self {
        match val {
            Emoji::Dice | Emoji::Dart => 1..=6,
            Emoji::Basketball | Emoji::Football => 1..=5,
            Emoji::SlotMachine => 1..=64,
        }
    }
}

impl From<Emoji> for Range<i64> {
    fn from(val: Emoji) -> Self {
        match val {
            Emoji::Dice | Emoji::Dart => 1..7,
            Emoji::Basketball | Emoji::Football => 1..6,
            Emoji::SlotMachine => 1..65,
        }
    }
}
