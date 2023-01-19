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

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum DiceEmoji {
    Dice,
    Dart,
    Basketball,
    Football,
    SlotMachine,
}

impl DiceEmoji {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            DiceEmoji::Dice => "🎲",
            DiceEmoji::Dart => "🎯",
            DiceEmoji::Basketball => "🏀",
            DiceEmoji::Football => "⚽",
            DiceEmoji::SlotMachine => "🎰",
        }
    }
}

impl From<DiceEmoji> for RangeInclusive<i64> {
    fn from(val: DiceEmoji) -> Self {
        match val {
            DiceEmoji::Dice | DiceEmoji::Dart => 1..=6,
            DiceEmoji::Basketball | DiceEmoji::Football => 1..=5,
            DiceEmoji::SlotMachine => 1..=64,
        }
    }
}

impl From<DiceEmoji> for Range<i64> {
    fn from(val: DiceEmoji) -> Self {
        match val {
            DiceEmoji::Dice | DiceEmoji::Dart => 1..7,
            DiceEmoji::Basketball | DiceEmoji::Football => 1..6,
            DiceEmoji::SlotMachine => 1..65,
        }
    }
}
