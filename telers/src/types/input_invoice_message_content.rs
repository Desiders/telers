use serde::{Deserialize, Serialize};
/// Represents the content of an invoice message to be sent as the result of an inline query.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputinvoicemessagecontent>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputInvoiceMessageContent {
    /// Product name, 1-32 characters
    pub title: Box<str>,
    /// Product description, 1-255 characters
    pub description: Box<str>,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use it for your internal processes.
    pub payload: Box<str>,
    /// Payment provider token, obtained via @`BotFather`. Pass an empty string for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<Box<str>>,
    /// Three-letter ISO 4217 currency code, see more on currencies. Pass `XTR` for payments in Telegram Stars.
    pub currency: Box<str>,
    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in Telegram Stars.
    pub prices: Box<[crate::types::LabeledPrice]>,
    /// The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double). For example, for a maximum tip of US$ 1.45 pass `max_tip_amount` = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i64>,
    /// A JSON-serialized array of suggested amounts of tip in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed `max_tip_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Box<[i64]>>,
    /// A JSON-serialized object for data about the invoice, which will be shared with the payment provider. A detailed description of the required fields should be provided by the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<Box<str>>,
    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<Box<str>>,
    /// Photo size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i64>,
    /// Photo width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i64>,
    /// Photo height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i64>,
    /// Pass `true` if you require the user's full name to complete the order. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    /// Pass `true` if you require the user's phone number to complete the order. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    /// Pass `true` if you require the user's email address to complete the order. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    /// Pass `true` if you require the user's shipping address to complete the order. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    /// Pass `true` if the user's phone number should be sent to the provider. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,
    /// Pass `true` if the user's email address should be sent to the provider. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,
    /// Pass `true` if the final price depends on the shipping method. Ignored for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
}
impl InputInvoiceMessageContent {
    /// Creates a new `InputInvoiceMessageContent`.
    ///
    /// # Arguments
    /// * `title` - Product name, 1-32 characters
    /// * `description` - Product description, 1-255 characters
    /// * `payload` - Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use it for your internal processes.
    /// * `currency` - Three-letter ISO 4217 currency code, see more on currencies. Pass `XTR` for payments in Telegram Stars.
    /// * `prices` - Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in Telegram Stars.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<Box<str>>,
        T2: Into<Box<str>>,
        T3: Into<Box<str>>,
        T4Item: Into<crate::types::LabeledPrice>,
        T4: IntoIterator<Item = T4Item>,
    >(
        title: T0,
        description: T1,
        payload: T2,
        currency: T3,
        prices: T4,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            payload: payload.into(),
            provider_token: None,
            currency: currency.into(),
            prices: prices.into_iter().map(Into::into).collect(),
            max_tip_amount: None,
            suggested_tip_amounts: None,
            provider_data: None,
            photo_url: None,
            photo_size: None,
            photo_width: None,
            photo_height: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
        }
    }

    /// Product name, 1-32 characters
    #[must_use]
    pub fn title<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.title = val.into();
        this
    }

    /// Product description, 1-255 characters
    #[must_use]
    pub fn description<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.description = val.into();
        this
    }

    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use it for your internal processes.
    #[must_use]
    pub fn payload<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.payload = val.into();
        this
    }

    /// Payment provider token, obtained via @`BotFather`. Pass an empty string for payments in Telegram Stars.
    #[must_use]
    pub fn provider_token<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.provider_token = Some(val.into());
        this
    }

    /// Payment provider token, obtained via @`BotFather`. Pass an empty string for payments in Telegram Stars.
    #[must_use]
    pub fn provider_token_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.provider_token = val.map(Into::into);
        this
    }

    /// Three-letter ISO 4217 currency code, see more on currencies. Pass `XTR` for payments in Telegram Stars.
    #[must_use]
    pub fn currency<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.currency = val.into();
        this
    }

    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in Telegram Stars.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn prices<T: Into<Box<[crate::types::LabeledPrice]>>>(self, val: T) -> Self {
        let mut this = self;
        this.prices = this
            .prices
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        this
    }

    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in Telegram Stars.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn price<T: Into<crate::types::LabeledPrice>>(self, val: T) -> Self {
        let mut this = self;
        this.prices = this
            .prices
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }

    /// The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double). For example, for a maximum tip of US$ 1.45 pass `max_tip_amount` = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in Telegram Stars.
    #[must_use]
    pub fn max_tip_amount<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.max_tip_amount = Some(val.into());
        this
    }

    /// The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double). For example, for a maximum tip of US$ 1.45 pass `max_tip_amount` = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in Telegram Stars.
    #[must_use]
    pub fn max_tip_amount_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.max_tip_amount = val.map(Into::into);
        this
    }

    /// A JSON-serialized array of suggested amounts of tip in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed `max_tip_amount`.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn suggested_tip_amounts<T: Into<Box<[i64]>>>(self, val: T) -> Self {
        let mut this = self;
        this.suggested_tip_amounts = Some(
            this.suggested_tip_amounts
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        this
    }

    /// A JSON-serialized array of suggested amounts of tip in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed `max_tip_amount`.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn suggested_tip_amount<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.suggested_tip_amounts = Some(
            this.suggested_tip_amounts
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// A JSON-serialized array of suggested amounts of tip in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed `max_tip_amount`.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn suggested_tip_amounts_option<T: Into<Box<[i64]>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.suggested_tip_amounts = val.map(Into::into);
        this
    }

    /// A JSON-serialized object for data about the invoice, which will be shared with the payment provider. A detailed description of the required fields should be provided by the payment provider.
    #[must_use]
    pub fn provider_data<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.provider_data = Some(val.into());
        this
    }

    /// A JSON-serialized object for data about the invoice, which will be shared with the payment provider. A detailed description of the required fields should be provided by the payment provider.
    #[must_use]
    pub fn provider_data_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.provider_data = val.map(Into::into);
        this
    }

    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    #[must_use]
    pub fn photo_url<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.photo_url = Some(val.into());
        this
    }

    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    #[must_use]
    pub fn photo_url_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.photo_url = val.map(Into::into);
        this
    }

    /// Photo size in bytes
    #[must_use]
    pub fn photo_size<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.photo_size = Some(val.into());
        this
    }

    /// Photo size in bytes
    #[must_use]
    pub fn photo_size_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.photo_size = val.map(Into::into);
        this
    }

    /// Photo width
    #[must_use]
    pub fn photo_width<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.photo_width = Some(val.into());
        this
    }

    /// Photo width
    #[must_use]
    pub fn photo_width_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.photo_width = val.map(Into::into);
        this
    }

    /// Photo height
    #[must_use]
    pub fn photo_height<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.photo_height = Some(val.into());
        this
    }

    /// Photo height
    #[must_use]
    pub fn photo_height_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.photo_height = val.map(Into::into);
        this
    }

    /// Pass `true` if you require the user's full name to complete the order. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn need_name<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.need_name = Some(val.into());
        this
    }

    /// Pass `true` if you require the user's full name to complete the order. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn need_name_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.need_name = val.map(Into::into);
        this
    }

    /// Pass `true` if you require the user's phone number to complete the order. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn need_phone_number<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.need_phone_number = Some(val.into());
        this
    }

    /// Pass `true` if you require the user's phone number to complete the order. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn need_phone_number_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.need_phone_number = val.map(Into::into);
        this
    }

    /// Pass `true` if you require the user's email address to complete the order. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn need_email<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.need_email = Some(val.into());
        this
    }

    /// Pass `true` if you require the user's email address to complete the order. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn need_email_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.need_email = val.map(Into::into);
        this
    }

    /// Pass `true` if you require the user's shipping address to complete the order. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn need_shipping_address<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.need_shipping_address = Some(val.into());
        this
    }

    /// Pass `true` if you require the user's shipping address to complete the order. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn need_shipping_address_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.need_shipping_address = val.map(Into::into);
        this
    }

    /// Pass `true` if the user's phone number should be sent to the provider. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn send_phone_number_to_provider<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.send_phone_number_to_provider = Some(val.into());
        this
    }

    /// Pass `true` if the user's phone number should be sent to the provider. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn send_phone_number_to_provider_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.send_phone_number_to_provider = val.map(Into::into);
        this
    }

    /// Pass `true` if the user's email address should be sent to the provider. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn send_email_to_provider<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.send_email_to_provider = Some(val.into());
        this
    }

    /// Pass `true` if the user's email address should be sent to the provider. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn send_email_to_provider_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.send_email_to_provider = val.map(Into::into);
        this
    }

    /// Pass `true` if the final price depends on the shipping method. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn is_flexible<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_flexible = Some(val.into());
        this
    }

    /// Pass `true` if the final price depends on the shipping method. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn is_flexible_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.is_flexible = val.map(Into::into);
        this
    }
}
