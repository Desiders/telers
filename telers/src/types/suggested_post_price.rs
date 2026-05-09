use serde::{Deserialize, Serialize};
/// Describes the price of a suggested post.
/// # Documentation
/// <https://core.telegram.org/bots/api#suggestedpostprice>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuggestedPostPrice {
    /// Currency in which the post will be paid. Currently, must be one of `XTR` for Telegram Stars or `TON` for toncoins
    pub currency: Box<str>,
    /// The amount of the currency that will be paid for the post in the smallest units of the currency, i.e. Telegram Stars or nanotoncoins. Currently, price in Telegram Stars must be between 5 and 100000, and price in nanotoncoins must be between 10000000 and 10000000000000.
    pub amount: u32,
}
impl SuggestedPostPrice {
    /// Creates a new `SuggestedPostPrice`.
    ///
    /// # Arguments
    /// * `currency` - Currency in which the post will be paid. Currently, must be one of `XTR` for Telegram Stars or `TON` for toncoins
    /// * `amount` - The amount of the currency that will be paid for the post in the smallest units of the currency, i.e. Telegram Stars or nanotoncoins. Currently, price in Telegram Stars must be between 5 and 100000, and price in nanotoncoins must be between 10000000 and 10000000000000.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<u32>>(currency: T0, amount: T1) -> Self {
        Self {
            currency: currency.into(),
            amount: amount.into(),
        }
    }

    /// Currency in which the post will be paid. Currently, must be one of `XTR` for Telegram Stars or `TON` for toncoins
    #[must_use]
    pub fn currency<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.currency = val.into();
        self
    }

    /// The amount of the currency that will be paid for the post in the smallest units of the currency, i.e. Telegram Stars or nanotoncoins. Currently, price in Telegram Stars must be between 5 and 100000, and price in nanotoncoins must be between 10000000 and 10000000000000.
    #[must_use]
    pub fn amount<T: Into<u32>>(mut self, val: T) -> Self {
        self.amount = val.into();
        self
    }
}
