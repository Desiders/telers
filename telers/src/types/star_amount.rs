use serde::Deserialize;

/// Describes an amount of Telegram Stars.
/// # Documentation
/// <https://core.telegram.org/bots/api#staramount>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct StarAmount {
    /// Integer amount of Telegram Stars, rounded to 0; can be negative
    pub amount: i64,
    /// The number of 1/1000000000 shares of Telegram Stars; from -999999999 to 999999999; can be negative if and only if amount is non-positive
    pub nanostar_amount: i64,
}
