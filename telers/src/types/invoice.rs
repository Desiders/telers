use serde::{Deserialize, Serialize};
/// This object contains basic information about an invoice.
/// # Documentation
/// <https://core.telegram.org/bots/api#invoice>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Invoice {
    /// Product name
    pub title: Box<str>,
    /// Product description
    pub description: Box<str>,
    /// Unique bot deep-linking parameter that can be used to generate this invoice
    pub start_parameter: Box<str>,
    /// Three-letter ISO 4217 currency code, or `XTR` for payments in Telegram Stars
    pub currency: Box<str>,
    /// Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: i64,
}
impl Invoice {
    /// Creates a new `Invoice`.
    ///
    /// # Arguments
    /// * `title` - Product name
    /// * `description` - Product description
    /// * `start_parameter` - Unique bot deep-linking parameter that can be used to generate this invoice
    /// * `currency` - Three-letter ISO 4217 currency code, or `XTR` for payments in Telegram Stars
    /// * `total_amount` - Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<Box<str>>,
        T2: Into<Box<str>>,
        T3: Into<Box<str>>,
        T4: Into<i64>,
    >(
        title: T0,
        description: T1,
        start_parameter: T2,
        currency: T3,
        total_amount: T4,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            start_parameter: start_parameter.into(),
            currency: currency.into(),
            total_amount: total_amount.into(),
        }
    }

    /// Product name
    #[must_use]
    pub fn title<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.title = val.into();
        this
    }

    /// Product description
    #[must_use]
    pub fn description<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.description = val.into();
        this
    }

    /// Unique bot deep-linking parameter that can be used to generate this invoice
    #[must_use]
    pub fn start_parameter<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.start_parameter = val.into();
        this
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
}
