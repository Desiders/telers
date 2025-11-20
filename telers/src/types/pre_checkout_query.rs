use super::{OrderInfo, Update, UpdateKind, User};

use crate::{errors::ConvertToTypeError, FromEvent};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object contains information about an incoming pre-checkout query.
/// # Documentation
/// <https://core.telegram.org/bots/api#precheckoutquery>
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize, FromEvent)]
#[event(try_from = Update)]
pub struct PreCheckoutQuery {
    /// Unique query identifier
    pub id: Box<str>,
    /// User who sent the query
    pub from: User,
    /// Three-letter ISO 4217 [`currency`](https://core.telegram.org/bots/payments#supported-currencies) code, or `XTR`for payments in [`Telegram Stars`](https://t.me/BotNews/90)
    pub currency: Box<str>,
    /// Total price in the *smallest units* of the currency (integer, **not** float/double). For example, for a price of `US$ 1.45` pass `amount = 145`. See the *exp* parameter in [`currencies.json`](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
    /// Bot specified invoice payload
    pub invoice_payload: Box<str>,
    /// Identifier of the shipping option chosen by the user
    pub shipping_option_id: Option<Box<str>>,
    /// Order info provided by the user
    pub order_info: Option<OrderInfo>,
}

impl TryFrom<Update> for PreCheckoutQuery {
    type Error = ConvertToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        match update.kind {
            UpdateKind::PreCheckoutQuery(val) => Ok(val),
            _ => Err(ConvertToTypeError::new("Update", "PreCheckoutQuery")),
        }
    }
}
