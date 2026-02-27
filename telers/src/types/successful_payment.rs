use serde::{Deserialize, Serialize};
/// This object contains basic information about a successful payment. Note that if the buyer initiates a chargeback with the relevant payment provider following this transaction, the funds may be debited from your balance. This is outside of Telegram's control.
/// # Documentation
/// <https://core.telegram.org/bots/api#successfulpayment>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuccessfulPayment {
    /// Three-letter ISO 4217 currency code, or `XTR` for payments in Telegram Stars
    pub currency: Box<str>,
    /// Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
    /// Bot-specified invoice payload
    pub invoice_payload: Box<str>,
    /// Expiration date of the subscription, in Unix time; for recurring payments only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_expiration_date: Option<i64>,
    /// `true`, if the payment is a recurring payment for a subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_recurring: Option<bool>,
    /// `true`, if the payment is the first payment for a subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_first_recurring: Option<bool>,
    /// Identifier of the shipping option chosen by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<Box<str>>,
    /// Order information provided by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<crate::types::OrderInfo>,
    /// Telegram payment identifier
    pub telegram_payment_charge_id: Box<str>,
    /// Provider payment identifier
    pub provider_payment_charge_id: Box<str>,
}
impl SuccessfulPayment {
    /// Creates a new `SuccessfulPayment`.
    ///
    /// # Arguments
    /// * `currency` - Three-letter ISO 4217 currency code, or `XTR` for payments in Telegram Stars
    /// * `total_amount` - Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    /// * `invoice_payload` - Bot-specified invoice payload
    /// * `telegram_payment_charge_id` - Telegram payment identifier
    /// * `provider_payment_charge_id` - Provider payment identifier
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<i64>,
        T2: Into<Box<str>>,
        T3: Into<Box<str>>,
        T4: Into<Box<str>>,
    >(
        currency: T0,
        total_amount: T1,
        invoice_payload: T2,
        telegram_payment_charge_id: T3,
        provider_payment_charge_id: T4,
    ) -> Self {
        Self {
            currency: currency.into(),
            total_amount: total_amount.into(),
            invoice_payload: invoice_payload.into(),
            subscription_expiration_date: None,
            is_recurring: None,
            is_first_recurring: None,
            shipping_option_id: None,
            order_info: None,
            telegram_payment_charge_id: telegram_payment_charge_id.into(),
            provider_payment_charge_id: provider_payment_charge_id.into(),
        }
    }

    /// Three-letter ISO 4217 currency code, or `XTR` for payments in Telegram Stars
    #[must_use]
    pub fn currency<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.currency = val.into();
        this
    }

    /// Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    #[must_use]
    pub fn total_amount<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.total_amount = val.into();
        this
    }

    /// Bot-specified invoice payload
    #[must_use]
    pub fn invoice_payload<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.invoice_payload = val.into();
        this
    }

    /// Expiration date of the subscription, in Unix time; for recurring payments only
    #[must_use]
    pub fn subscription_expiration_date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.subscription_expiration_date = Some(val.into());
        this
    }

    /// Expiration date of the subscription, in Unix time; for recurring payments only
    #[must_use]
    pub fn subscription_expiration_date_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.subscription_expiration_date = val.map(Into::into);
        this
    }

    /// `true`, if the payment is a recurring payment for a subscription
    #[must_use]
    pub fn is_recurring<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_recurring = Some(val.into());
        this
    }

    /// `true`, if the payment is a recurring payment for a subscription
    #[must_use]
    pub fn is_recurring_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.is_recurring = val.map(Into::into);
        this
    }

    /// `true`, if the payment is the first payment for a subscription
    #[must_use]
    pub fn is_first_recurring<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_first_recurring = Some(val.into());
        this
    }

    /// `true`, if the payment is the first payment for a subscription
    #[must_use]
    pub fn is_first_recurring_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.is_first_recurring = val.map(Into::into);
        this
    }

    /// Identifier of the shipping option chosen by the user
    #[must_use]
    pub fn shipping_option_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.shipping_option_id = Some(val.into());
        this
    }

    /// Identifier of the shipping option chosen by the user
    #[must_use]
    pub fn shipping_option_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.shipping_option_id = val.map(Into::into);
        this
    }

    /// Order information provided by the user
    #[must_use]
    pub fn order_info<T: Into<crate::types::OrderInfo>>(self, val: T) -> Self {
        let mut this = self;
        this.order_info = Some(val.into());
        this
    }

    /// Order information provided by the user
    #[must_use]
    pub fn order_info_option<T: Into<crate::types::OrderInfo>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.order_info = val.map(Into::into);
        this
    }

    /// Telegram payment identifier
    #[must_use]
    pub fn telegram_payment_charge_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.telegram_payment_charge_id = val.into();
        this
    }

    /// Provider payment identifier
    #[must_use]
    pub fn provider_payment_charge_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.provider_payment_charge_id = val.into();
        this
    }
}
