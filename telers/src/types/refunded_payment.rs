use serde::{Deserialize, Serialize};
/// This object contains basic information about a refunded payment.
/// # Documentation
/// <https://core.telegram.org/bots/api#refundedpayment>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RefundedPayment {
    /// Three-letter ISO 4217 currency code, or `XTR` for payments in Telegram Stars. Currently, always `XTR`
    pub currency: Box<str>,
    /// Total refunded price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45, `total_amount` = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
    /// Bot-specified invoice payload
    pub invoice_payload: Box<str>,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: Box<str>,
    /// Provider payment identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_payment_charge_id: Option<Box<str>>,
}
impl RefundedPayment {
    /// Creates a new `RefundedPayment`.
    ///
    /// # Arguments
    /// * `currency` - Three-letter ISO 4217 currency code, or `XTR` for payments in Telegram Stars. Currently, always `XTR`
    /// * `total_amount` - Total refunded price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45, `total_amount` = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    /// * `invoice_payload` - Bot-specified invoice payload
    /// * `telegram_payment_charge_id` - Telegram payment identifier
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<i64>, T2: Into<Box<str>>, T3: Into<Box<str>>>(
        currency: T0,
        total_amount: T1,
        invoice_payload: T2,
        telegram_payment_charge_id: T3,
    ) -> Self {
        Self {
            currency: currency.into(),
            total_amount: total_amount.into(),
            invoice_payload: invoice_payload.into(),
            telegram_payment_charge_id: telegram_payment_charge_id.into(),
            provider_payment_charge_id: None,
        }
    }

    /// Three-letter ISO 4217 currency code, or `XTR` for payments in Telegram Stars. Currently, always `XTR`
    #[must_use]
    pub fn currency<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.currency = val.into();
        self
    }

    /// Total refunded price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45, `total_amount` = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
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

    /// Telegram payment identifier
    #[must_use]
    pub fn telegram_payment_charge_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.telegram_payment_charge_id = val.into();
        self
    }

    /// Provider payment identifier
    #[must_use]
    pub fn provider_payment_charge_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.provider_payment_charge_id = Some(val.into());
        self
    }

    /// Provider payment identifier
    #[must_use]
    pub fn provider_payment_charge_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.provider_payment_charge_id = val.map(Into::into);
        self
    }
}
