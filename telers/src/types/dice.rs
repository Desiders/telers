use serde::{Deserialize, Serialize};
/// This object represents an animated emoji that displays a random value.
/// # Documentation
/// <https://core.telegram.org/bots/api#dice>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dice {
    /// Emoji on which the dice throw animation is based
    pub emoji: Box<str>,
    /// Value of the dice, 1-6 for `🎲`, `🎯` and `🎳` base emoji, 1-5 for `🏀` and `⚽` base emoji, 1-64 for `🎰` base emoji
    pub value: u8,
}
impl Dice {
    /// Creates a new `Dice`.
    ///
    /// # Arguments
    /// * `emoji` - Emoji on which the dice throw animation is based
    /// * `value` - Value of the dice, 1-6 for `🎲`, `🎯` and `🎳` base emoji, 1-5 for `🏀` and `⚽` base emoji, 1-64 for `🎰` base emoji
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<u8>>(emoji: T0, value: T1) -> Self {
        Self {
            emoji: emoji.into(),
            value: value.into(),
        }
    }

    /// Emoji on which the dice throw animation is based
    #[must_use]
    pub fn emoji<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.emoji = val.into();
        self
    }

    /// Value of the dice, 1-6 for `🎲`, `🎯` and `🎳` base emoji, 1-5 for `🏀` and `⚽` base emoji, 1-64 for `🎰` base emoji
    #[must_use]
    pub fn value<T: Into<u8>>(mut self, val: T) -> Self {
        self.value = val.into();
        self
    }
}
