use serde::{Deserialize, Serialize};
/// This object represents a portion of the price for goods or services.
/// # Documentation
/// <https://core.telegram.org/bots/api#labeledprice>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LabeledPrice {
    /// Portion label
    pub label: Box<str>,
    /// Price of the product in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub amount: i64,
}
impl LabeledPrice {
    /// Creates a new `LabeledPrice`.
    ///
    /// # Arguments
    /// * `label` - Portion label
    /// * `amount` - Price of the product in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<i64>>(label: T0, amount: T1) -> Self {
        Self {
            label: label.into(),
            amount: amount.into(),
        }
    }

    /// Portion label
    #[must_use]
    pub fn label<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.label = val.into();
        self
    }

    /// Price of the product in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    #[must_use]
    pub fn amount<T: Into<i64>>(mut self, val: T) -> Self {
        self.amount = val.into();
        self
    }
}
