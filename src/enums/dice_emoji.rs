use std::ops::{Range, RangeInclusive};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// This enum represents all possible types of the dice emoji
/// # Documentation
/// <https://core.telegram.org/bots/api#dice>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum DiceEmoji {
    #[strum(serialize = "🎲")]
    Dice,
    #[strum(serialize = "🎯")]
    Dart,
    #[strum(serialize = "🏀")]
    Basketball,
    #[strum(serialize = "⚽")]
    Football,
    #[strum(serialize = "🎰")]
    SlotMachine,
    #[strum(serialize = "🎳")]
    Bowling,
}

impl DiceEmoji {
    #[must_use]
    pub const fn all() -> [DiceEmoji; 6] {
        [
            DiceEmoji::Dice,
            DiceEmoji::Dart,
            DiceEmoji::Basketball,
            DiceEmoji::Football,
            DiceEmoji::SlotMachine,
            DiceEmoji::Bowling,
        ]
    }
}

impl From<DiceEmoji> for RangeInclusive<i64> {
    fn from(dice_emoji: DiceEmoji) -> Self {
        match dice_emoji {
            DiceEmoji::Dice | DiceEmoji::Dart | DiceEmoji::Bowling => 1..=6,
            DiceEmoji::Basketball | DiceEmoji::Football => 1..=5,
            DiceEmoji::SlotMachine => 1..=64,
        }
    }
}

impl From<DiceEmoji> for Range<i64> {
    fn from(dice_emoji: DiceEmoji) -> Self {
        match dice_emoji {
            DiceEmoji::Dice | DiceEmoji::Dart | DiceEmoji::Bowling => 1..7,
            DiceEmoji::Basketball | DiceEmoji::Football => 1..6,
            DiceEmoji::SlotMachine => 1..65,
        }
    }
}

impl From<DiceEmoji> for Box<str> {
    fn from(dice_emoji: DiceEmoji) -> Self {
        Into::<&'static str>::into(dice_emoji).into()
    }
}

impl From<DiceEmoji> for String {
    fn from(dice_emoji: DiceEmoji) -> Self {
        dice_emoji.as_ref().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for DiceEmoji {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
