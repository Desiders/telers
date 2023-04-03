use std::{
    fmt::{self, Debug},
    ops::{Range, RangeInclusive},
};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
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

impl Debug for DiceEmoji {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
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

impl From<DiceEmoji> for String {
    fn from(val: DiceEmoji) -> Self {
        val.as_str().to_string()
    }
}
