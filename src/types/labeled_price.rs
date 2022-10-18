use serde::{Deserialize, Serialize};

/// This object represents a portion of the price for goods or services.
/// <https://core.telegram.org/bots/api#labeledprice>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct LabeledPrice {
    /// Portion label
    pub label: String,
    /// Price of the product in the *smallest units* of the `currency <https://core.telegram.org/bots/payments#supported-currencies>`_ (integer, **not** float/double). For example, for a price of :code:`US$ 1.45` pass :code:`amount = 145`. See the *exp* parameter in `currencies.json <https://core.telegram.org/bots/payments/currencies.json>`_, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub amount: i64,
}

impl Default for LabeledPrice {
    fn default() -> Self {
        Self {
            label: String::default(),
            amount: 0,
        }
    }
}
