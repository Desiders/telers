use super::{OrderInfo, Update, User};

use crate::errors::ConvertUpdateToTypeError;

use serde::Deserialize;

/// This object contains information about an incoming pre-checkout query.
/// # Documentation
/// <https://core.telegram.org/bots/api#precheckoutquery>
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct PreCheckoutQuery {
    /// Unique query identifier
    pub id: Box<str>,
    /// User who sent the query
    pub from: User,
    /// Three-letter ISO 4217 [`currency`](https://core.telegram.org/bots/payments#supported-currencies) code
    pub currency: Box<str>,
    /// Total price in the *smallest units* of the currency (integer, **not** float/double). For example, for a price of `US$ 1.45` pass `amount = 145`. See the *exp* parameter in [`currencies.json`](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i32,
    /// Bot specified invoice payload
    pub invoice_payload: Box<str>,
    /// Identifier of the shipping option chosen by the user
    pub shipping_option_id: Option<Box<str>>,
    /// Order info provided by the user
    pub order_info: Option<OrderInfo>,
}

impl PreCheckoutQuery {
    /// Gets the sender user ID from the pre-checkout query
    #[must_use]
    pub const fn sender_user_id(&self) -> i64 {
        self.from.id
    }

    /// Gets the sender user ID from the pre-checkout query
    /// # Notes
    /// Alias to `sender_user_id` method
    #[must_use]
    pub const fn user_id(&self) -> i64 {
        self.sender_user_id()
    }
}

impl TryFrom<Update> for PreCheckoutQuery {
    type Error = ConvertUpdateToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        if let Some(pre_checkout_query) = update.pre_checkout_query {
            Ok(pre_checkout_query)
        } else {
            Err(ConvertUpdateToTypeError::new("PreCheckoutQuery"))
        }
    }
}
