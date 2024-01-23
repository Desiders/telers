use serde::Deserialize;

/// This object represents an animated emoji that displays a random value.
/// # Documentation
/// <https://core.telegram.org/bots/api#dice>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct Dice {
    /// Emoji on which the dice throw animation is based
    pub emoji: Box<str>,
    /// Value of the dice, 1-6 for '🎲', '🎯' and '🎳' base emoji, 1-5 for '🏀' and '⚽' base emoji, 1-64 for '🎰' base emoji
    pub value: i64,
}
