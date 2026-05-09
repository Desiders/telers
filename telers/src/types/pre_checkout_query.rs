use serde::{Deserialize, Serialize};
/// This object contains information about an incoming pre-checkout query.
/// # Documentation
/// <https://core.telegram.org/bots/api#precheckoutquery>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PreCheckoutQuery {
    /// Unique query identifier
    pub id: Box<str>,
    /// User who sent the query
    pub from: Box<crate::types::User>,
    /// Three-letter ISO 4217 currency code, or `XTR` for payments in Telegram Stars
    pub currency: Box<str>,
    /// Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
    /// Bot-specified invoice payload
    pub invoice_payload: Box<str>,
    /// Identifier of the shipping option chosen by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<Box<str>>,
    /// Order information provided by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<crate::types::OrderInfo>,
}
impl PreCheckoutQuery {
    /// Creates a new `PreCheckoutQuery`.
    ///
    /// # Arguments
    /// * `id` - Unique query identifier
    /// * `from` - User who sent the query
    /// * `currency` - Three-letter ISO 4217 currency code, or `XTR` for payments in Telegram Stars
    /// * `total_amount` - Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    /// * `invoice_payload` - Bot-specified invoice payload
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<crate::types::User>,
        T2: Into<Box<str>>,
        T3: Into<i64>,
        T4: Into<Box<str>>,
    >(
        id: T0,
        from: T1,
        currency: T2,
        total_amount: T3,
        invoice_payload: T4,
    ) -> Self {
        Self {
            id: id.into(),
            from: Box::new(from.into()),
            currency: currency.into(),
            total_amount: total_amount.into(),
            invoice_payload: invoice_payload.into(),
            shipping_option_id: None,
            order_info: None,
        }
    }

    /// Unique query identifier
    #[must_use]
    pub fn id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// User who sent the query
    #[must_use]
    pub fn from<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.from = Box::new(val.into());
        self
    }

    /// Three-letter ISO 4217 currency code, or `XTR` for payments in Telegram Stars
    #[must_use]
    pub fn currency<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.currency = val.into();
        self
    }

    /// Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    #[must_use]
    pub fn total_amount<T: Into<i64>>(mut self, val: T) -> Self {
        self.total_amount = val.into();
        self
    }

    /// Bot-specified invoice payload
    #[must_use]
    pub fn invoice_payload<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.invoice_payload = val.into();
        self
    }

    /// Identifier of the shipping option chosen by the user
    #[must_use]
    pub fn shipping_option_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.shipping_option_id = Some(val.into());
        self
    }

    /// Identifier of the shipping option chosen by the user
    #[must_use]
    pub fn shipping_option_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.shipping_option_id = val.map(Into::into);
        self
    }

    /// Order information provided by the user
    #[must_use]
    pub fn order_info<T: Into<crate::types::OrderInfo>>(mut self, val: T) -> Self {
        self.order_info = Some(val.into());
        self
    }

    /// Order information provided by the user
    #[must_use]
    pub fn order_info_option<T: Into<crate::types::OrderInfo>>(mut self, val: Option<T>) -> Self {
        self.order_info = val.map(Into::into);
        self
    }
}
