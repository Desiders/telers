use super::OrderInfo;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object contains basic information about a successful payment.
/// # Documentation
/// <https://core.telegram.org/bots/api#successfulpayment>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct SuccessfulPayment {
    /// Three-letter ISO 4217 [`currency`](https://core.telegram.org/bots/payments#supported-currencies) code, or `XTR`` for payments in [`Telegram Stars`](https://t.me/BotNews/90)
    pub currency: Box<str>,
    /// Total price in the *smallest units* of the currency (integer, **not** float/double). For example, for a price of `US$ 1.45` pass `amount = 145`. See the *exp* parameter in [`currencies.json`](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
    /// Bot specified invoice payload
    pub invoice_payload: Box<str>,
    /// Expiration date of the subscription, in Unix time; for recurring payments only
    pub subscription_expiration_date: Option<i64>,
    /// `true`, if the payment is a recurring payment for a subscription
    pub is_recurring: Option<bool>,
    /// `true`, if the payment is the first payment for a subscription
    pub is_first_recurring: Option<bool>,
    /// Identifier of the shipping option chosen by the user
    pub shipping_option_id: Option<Box<str>>,
    /// Order info provided by the user
    pub order_info: Option<OrderInfo>,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: Box<str>,
    /// Provider payment identifier
    pub provider_payment_charge_id: Box<str>,
}
