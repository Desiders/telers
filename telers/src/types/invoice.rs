use serde::{Deserialize, Serialize};

/// This object contains basic information about an invoice.
/// # Documentation
/// <https://core.telegram.org/bots/api#invoice>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct Invoice {
    /// Product name
    pub title: Box<str>,
    /// Product description
    pub description: Box<str>,
    /// Unique bot deep-linking parameter that can be used to generate this invoice
    pub start_parameter: Box<str>,
    /// Three-letter ISO 4217 [`currency`](https://core.telegram.org/bots/payments#supported-currencies) code, or `XTR` for payments in [`Telegram Stars`](https://t.me/BotNews/90)
    pub currency: Box<str>,
    /// Total price in the *smallest units* of the currency (integer, **not** float/double). For example, for a price of `US$ 1.45` pass `amount = 145`. See the *exp* parameter in [`currencies.json`](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
}
