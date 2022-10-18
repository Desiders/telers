use super::OrderInfo;

use serde::{Deserialize, Serialize};

/// This object contains basic information about a successful payment.
/// <https://core.telegram.org/bots/api#successfulpayment>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SuccessfulPayment {
    /// Three-letter ISO 4217 `currency <https://core.telegram.org/bots/payments#supported-currencies>`_ code
    pub currency: String,
    /// Total price in the *smallest units* of the currency (integer, **not** float/double). For example, for a price of :code:`US$ 1.45` pass :code:`amount = 145`. See the *exp* parameter in `currencies.json <https://core.telegram.org/bots/payments/currencies.json>`_, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// *Optional*. Identifier of the shipping option chosen by the user
    pub shipping_option_id: Option<String>,
    /// *Optional*. Order info provided by the user
    pub order_info: Option<OrderInfo>,
    /// *Optional*. Telegram payment identifier
    pub telegram_payment_charge_id: String,
    /// Provider payment identifier
    pub provider_payment_charge_id: String,
}

impl Default for SuccessfulPayment {
    fn default() -> Self {
        Self {
            currency: String::default(),
            total_amount: 0,
            invoice_payload: String::default(),
            shipping_option_id: None,
            order_info: None,
            telegram_payment_charge_id: String::default(),
            provider_payment_charge_id: String::default(),
        }
    }
}
