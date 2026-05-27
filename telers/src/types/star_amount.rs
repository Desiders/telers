use serde::{Deserialize, Serialize};
/// Describes an amount of Telegram Stars.
/// # Documentation
/// <https://core.telegram.org/bots/api#staramount>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StarAmount {
    /// Integer amount of Telegram Stars, rounded to 0; can be negative
    pub amount: i64,
    /// The number of 1/1000000000 shares of Telegram Stars; from -999999999 to 999999999; can be negative if and only if amount is non-positive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nanostar_amount: Option<i32>,
}
impl StarAmount {
    /// Creates a new `StarAmount`.
    ///
    /// # Arguments
    /// * `amount` - Integer amount of Telegram Stars, rounded to 0; can be negative
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>>(amount: T0) -> Self {
        Self {
            amount: amount.into(),
            nanostar_amount: None,
        }
    }

    /// Integer amount of Telegram Stars, rounded to 0; can be negative
    #[must_use]
    pub fn amount<T: Into<i64>>(mut self, val: T) -> Self {
        self.amount = val.into();
        self
    }

    /// The number of 1/1000000000 shares of Telegram Stars; from -999999999 to 999999999; can be negative if and only if amount is non-positive
    #[must_use]
    pub fn nanostar_amount<T: Into<i32>>(mut self, val: T) -> Self {
        self.nanostar_amount = Some(val.into());
        self
    }

    /// The number of 1/1000000000 shares of Telegram Stars; from -999999999 to 999999999; can be negative if and only if amount is non-positive
    #[must_use]
    pub fn nanostar_amount_option<T: Into<i32>>(mut self, val: Option<T>) -> Self {
        self.nanostar_amount = val.map(Into::into);
        self
    }
}
