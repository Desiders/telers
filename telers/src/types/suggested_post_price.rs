use serde::{Deserialize, Serialize};
/// Describes the price of a suggested post.
/// # Documentation
/// <https://core.telegram.org/bots/api#suggestedpostprice>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedPostPrice {
    /// Currency in which the post will be paid. Currently, must be one of `XTR` for Telegram Stars or `TON` for TON grams.
    pub currency: Box<str>,
    /// The amount of the currency that will be paid for the post in the smallest units of the currency, i.e. Telegram Stars or nanograms. Currently, price in Telegram Stars must be between 5 and 100000, and price in nanograms must be between 10000000 and 10000000000000.
    pub amount: u64,
}
impl SuggestedPostPrice {
    /// Creates a new `SuggestedPostPrice`.
    ///
    /// # Arguments
    /// * `currency` - Currency in which the post will be paid. Currently, must be one of `XTR` for Telegram Stars or `TON` for TON grams.
    /// * `amount` - The amount of the currency that will be paid for the post in the smallest units of the currency, i.e. Telegram Stars or nanograms. Currently, price in Telegram Stars must be between 5 and 100000, and price in nanograms must be between 10000000 and 10000000000000.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<u64>>(currency: T0, amount: T1) -> Self {
        Self {
            currency: currency.into(),
            amount: amount.into(),
        }
    }

    /// Currency in which the post will be paid. Currently, must be one of `XTR` for Telegram Stars or `TON` for TON grams.
    #[must_use]
    pub fn currency<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.currency = val.into();
        self
    }

    /// The amount of the currency that will be paid for the post in the smallest units of the currency, i.e. Telegram Stars or nanograms. Currently, price in Telegram Stars must be between 5 and 100000, and price in nanograms must be between 10000000 and 10000000000000.
    #[must_use]
    pub fn amount<T: Into<u64>>(mut self, val: T) -> Self {
        self.amount = val.into();
        self
    }
}
