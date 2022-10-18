use serde::{Deserialize, Serialize};

/// This object represents an animated emoji that displays a random value.
/// <https://core.telegram.org/bots/api#dice>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Dice {
    /// Emoji on which the dice throw animation is based
    pub emoji: String,
    /// Value of the dice, 1-6 for '🎲', '🎯' and '🎳' base emoji, 1-5 for '🏀' and '⚽' base emoji, 1-64 for '🎰' base emoji
    pub value: i64,
}

#[allow(clippy::module_name_repetitions)]
pub enum DiceEmoji {
    Dice,
    Dart,
    Basketball,
    Football,
    SlotMachine,
}

impl DiceEmoji {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            DiceEmoji::Dice => "🎲",
            DiceEmoji::Dart => "🎯",
            DiceEmoji::Basketball => "🏀",
            DiceEmoji::Football => "⚽",
            DiceEmoji::SlotMachine => "🎰",
        }
    }
}
