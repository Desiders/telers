use std::{
    fmt::{self, Debug, Display},
    ops::{Deref, Range, RangeInclusive},
};

/// This enum represents all possible types of the dice emoji
/// # Documentation
/// <https://core.telegram.org/bots/api#dice>
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DiceEmoji {
    Dice,
    Dart,
    Basketball,
    Football,
    SlotMachine,
    Bowling,
}

impl DiceEmoji {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            DiceEmoji::Dice => "🎲",
            DiceEmoji::Dart => "🎯",
            DiceEmoji::Basketball => "🏀",
            DiceEmoji::Football => "⚽",
            DiceEmoji::SlotMachine => "🎰",
            DiceEmoji::Bowling => "🎳",
        }
    }
}

impl From<DiceEmoji> for RangeInclusive<i64> {
    fn from(val: DiceEmoji) -> Self {
        match val {
            DiceEmoji::Dice | DiceEmoji::Dart | DiceEmoji::Bowling => 1..=6,
            DiceEmoji::Basketball | DiceEmoji::Football => 1..=5,
            DiceEmoji::SlotMachine => 1..=64,
        }
    }
}

impl From<DiceEmoji> for Range<i64> {
    fn from(val: DiceEmoji) -> Self {
        match val {
            DiceEmoji::Dice | DiceEmoji::Dart | DiceEmoji::Bowling => 1..7,
            DiceEmoji::Basketball | DiceEmoji::Football => 1..6,
            DiceEmoji::SlotMachine => 1..65,
        }
    }
}

impl Deref for DiceEmoji {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Display for DiceEmoji {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<DiceEmoji> for Box<str> {
    fn from(val: DiceEmoji) -> Self {
        val.into()
    }
}

impl From<DiceEmoji> for String {
    fn from(val: DiceEmoji) -> Self {
        val.as_str().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for DiceEmoji {
    fn eq(&self, other: &&'a str) -> bool {
        self == other
    }
}
