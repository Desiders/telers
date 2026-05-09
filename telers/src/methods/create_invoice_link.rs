use crate::client::Bot;
use serde::Serialize;
/// Use this method to create a link for an invoice. Returns the created invoice link as String on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#createinvoicelink>
/// # Returns
/// - `Box<str>`
#[derive(Clone, Debug, Serialize)]
pub struct CreateInvoiceLink {
    /// Unique identifier of the business connection on behalf of which the link will be created. For payments in Telegram Stars only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<Box<str>>,
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
    /// The number of seconds the subscription will be active for before the next payment. The currency must be set to `XTR` (Telegram Stars) if the parameter is used. Currently, it must always be 2592000 (30 days) if specified. Any number of subscriptions can be active for a given bot at the same time, including multiple concurrent subscriptions from the same user. Subscription price must no exceed 10000 Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<i64>,
    /// The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double). For example, for a maximum tip of US$ 1.45 pass `max_tip_amount` = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in Telegram Stars.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i64>,
    /// A JSON-serialized array of suggested amounts of tips in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed `max_tip_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Box<[i64]>>,
    /// JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
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
impl CreateInvoiceLink {
    /// Creates a new `CreateInvoiceLink`.
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
            business_connection_id: None,
            title: title.into(),
            description: description.into(),
            payload: payload.into(),
            provider_token: None,
            currency: currency.into(),
            prices: prices.into_iter().map(Into::into).collect(),
            subscription_period: None,
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

    /// Unique identifier of the business connection on behalf of which the link will be created. For payments in Telegram Stars only.
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }

    /// Unique identifier of the business connection on behalf of which the link will be created. For payments in Telegram Stars only.
    #[must_use]
    pub fn business_connection_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.business_connection_id = val.map(Into::into);
        self
    }

    /// Product name, 1-32 characters
    #[must_use]
    pub fn title<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.title = val.into();
        self
    }

    /// Product description, 1-255 characters
    #[must_use]
    pub fn description<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.description = val.into();
        self
    }

    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use it for your internal processes.
    #[must_use]
    pub fn payload<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.payload = val.into();
        self
    }

    /// Payment provider token, obtained via @`BotFather`. Pass an empty string for payments in Telegram Stars.
    #[must_use]
    pub fn provider_token<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.provider_token = Some(val.into());
        self
    }

    /// Payment provider token, obtained via @`BotFather`. Pass an empty string for payments in Telegram Stars.
    #[must_use]
    pub fn provider_token_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.provider_token = val.map(Into::into);
        self
    }

    /// Three-letter ISO 4217 currency code, see more on currencies. Pass `XTR` for payments in Telegram Stars.
    #[must_use]
    pub fn currency<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.currency = val.into();
        self
    }

    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in Telegram Stars.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn prices<TItem: Into<crate::types::LabeledPrice>, T: IntoIterator<Item = TItem>>(
        mut self,
        val: T,
    ) -> Self {
        self.prices = self
            .prices
            .into_vec()
            .into_iter()
            .chain(val.into_iter().map(Into::into))
            .collect();
        self
    }

    /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in Telegram Stars.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn price<T: Into<crate::types::LabeledPrice>>(mut self, val: T) -> Self {
        self.prices = self
            .prices
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// The number of seconds the subscription will be active for before the next payment. The currency must be set to `XTR` (Telegram Stars) if the parameter is used. Currently, it must always be 2592000 (30 days) if specified. Any number of subscriptions can be active for a given bot at the same time, including multiple concurrent subscriptions from the same user. Subscription price must no exceed 10000 Telegram Stars.
    #[must_use]
    pub fn subscription_period<T: Into<i64>>(mut self, val: T) -> Self {
        self.subscription_period = Some(val.into());
        self
    }

    /// The number of seconds the subscription will be active for before the next payment. The currency must be set to `XTR` (Telegram Stars) if the parameter is used. Currently, it must always be 2592000 (30 days) if specified. Any number of subscriptions can be active for a given bot at the same time, including multiple concurrent subscriptions from the same user. Subscription price must no exceed 10000 Telegram Stars.
    #[must_use]
    pub fn subscription_period_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.subscription_period = val.map(Into::into);
        self
    }

    /// The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double). For example, for a maximum tip of US$ 1.45 pass `max_tip_amount` = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in Telegram Stars.
    #[must_use]
    pub fn max_tip_amount<T: Into<i64>>(mut self, val: T) -> Self {
        self.max_tip_amount = Some(val.into());
        self
    }

    /// The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double). For example, for a maximum tip of US$ 1.45 pass `max_tip_amount` = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in Telegram Stars.
    #[must_use]
    pub fn max_tip_amount_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.max_tip_amount = val.map(Into::into);
        self
    }

    /// A JSON-serialized array of suggested amounts of tips in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed `max_tip_amount`.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn suggested_tip_amounts<TItem: Into<i64>, T: IntoIterator<Item = TItem>>(
        mut self,
        val: T,
    ) -> Self {
        self.suggested_tip_amounts = Some(
            self.suggested_tip_amounts
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        self
    }

    /// A JSON-serialized array of suggested amounts of tips in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed `max_tip_amount`.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn suggested_tip_amount<T: Into<i64>>(mut self, val: T) -> Self {
        self.suggested_tip_amounts = Some(
            self.suggested_tip_amounts
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// A JSON-serialized array of suggested amounts of tips in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed `max_tip_amount`.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn suggested_tip_amounts_option<TItem: Into<i64>, T: IntoIterator<Item = TItem>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.suggested_tip_amounts = val.map(|v| v.into_iter().map(Into::into).collect());
        self
    }

    /// JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    #[must_use]
    pub fn provider_data<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.provider_data = Some(val.into());
        self
    }

    /// JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    #[must_use]
    pub fn provider_data_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.provider_data = val.map(Into::into);
        self
    }

    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    #[must_use]
    pub fn photo_url<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.photo_url = Some(val.into());
        self
    }

    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    #[must_use]
    pub fn photo_url_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.photo_url = val.map(Into::into);
        self
    }

    /// Photo size in bytes
    #[must_use]
    pub fn photo_size<T: Into<i64>>(mut self, val: T) -> Self {
        self.photo_size = Some(val.into());
        self
    }

    /// Photo size in bytes
    #[must_use]
    pub fn photo_size_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.photo_size = val.map(Into::into);
        self
    }

    /// Photo width
    #[must_use]
    pub fn photo_width<T: Into<i64>>(mut self, val: T) -> Self {
        self.photo_width = Some(val.into());
        self
    }

    /// Photo width
    #[must_use]
    pub fn photo_width_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.photo_width = val.map(Into::into);
        self
    }

    /// Photo height
    #[must_use]
    pub fn photo_height<T: Into<i64>>(mut self, val: T) -> Self {
        self.photo_height = Some(val.into());
        self
    }

    /// Photo height
    #[must_use]
    pub fn photo_height_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.photo_height = val.map(Into::into);
        self
    }

    /// Pass `true` if you require the user's full name to complete the order. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn need_name<T: Into<bool>>(mut self, val: T) -> Self {
        self.need_name = Some(val.into());
        self
    }

    /// Pass `true` if you require the user's full name to complete the order. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn need_name_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.need_name = val.map(Into::into);
        self
    }

    /// Pass `true` if you require the user's phone number to complete the order. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn need_phone_number<T: Into<bool>>(mut self, val: T) -> Self {
        self.need_phone_number = Some(val.into());
        self
    }

    /// Pass `true` if you require the user's phone number to complete the order. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn need_phone_number_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.need_phone_number = val.map(Into::into);
        self
    }

    /// Pass `true` if you require the user's email address to complete the order. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn need_email<T: Into<bool>>(mut self, val: T) -> Self {
        self.need_email = Some(val.into());
        self
    }

    /// Pass `true` if you require the user's email address to complete the order. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn need_email_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.need_email = val.map(Into::into);
        self
    }

    /// Pass `true` if you require the user's shipping address to complete the order. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn need_shipping_address<T: Into<bool>>(mut self, val: T) -> Self {
        self.need_shipping_address = Some(val.into());
        self
    }

    /// Pass `true` if you require the user's shipping address to complete the order. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn need_shipping_address_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.need_shipping_address = val.map(Into::into);
        self
    }

    /// Pass `true` if the user's phone number should be sent to the provider. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn send_phone_number_to_provider<T: Into<bool>>(mut self, val: T) -> Self {
        self.send_phone_number_to_provider = Some(val.into());
        self
    }

    /// Pass `true` if the user's phone number should be sent to the provider. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn send_phone_number_to_provider_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.send_phone_number_to_provider = val.map(Into::into);
        self
    }

    /// Pass `true` if the user's email address should be sent to the provider. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn send_email_to_provider<T: Into<bool>>(mut self, val: T) -> Self {
        self.send_email_to_provider = Some(val.into());
        self
    }

    /// Pass `true` if the user's email address should be sent to the provider. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn send_email_to_provider_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.send_email_to_provider = val.map(Into::into);
        self
    }

    /// Pass `true` if the final price depends on the shipping method. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn is_flexible<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_flexible = Some(val.into());
        self
    }

    /// Pass `true` if the final price depends on the shipping method. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn is_flexible_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_flexible = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for CreateInvoiceLink {
    type Method = Self;
    type Return = Box<str>;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("createInvoiceLink", self, None)
    }
}
