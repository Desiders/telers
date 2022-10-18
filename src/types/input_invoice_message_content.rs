use super::LabeledPrice;

use serde::{Deserialize, Serialize};

/// Represents the `content <https://core.telegram.org/bots/api#inputmessagecontent>`_ of an invoice message to be sent as the result of an inline query.
/// <https://core.telegram.org/bots/api#inputinvoicemessagecontent>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InputInvoiceMessageContent {
    /// Product name, 1-32 characters
    pub title: String,
    /// Product description, 1-255 characters
    pub description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for your internal processes.
    pub payload: String,
    /// Payments provider token, obtained via `Botfather <https://t.me/botfather>`
    pub provider_token: String,
    /// Three-letter ISO 4217 currency code, see `more on currencies <https://core.telegram.org/bots/payments#supported-currencies>`
    pub currency: String,
    /// Price breakdown, a list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)
    pub prices: Vec<LabeledPrice>,
    /// *Optional*. The maximum accepted amount for tips in the *smallest units* of the currency (integer, **not** float/double). For example, for a maximum tip of :code:`US$ 1.45` pass :code:`max_tip_amount = 145`. See the *exp* parameter in `currencies.json <https://core.telegram.org/bots/payments/currencies.json>`_, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0
    pub max_tip_amount: Option<i64>,
    /// *Optional*. A JSON-serialized array of suggested amounts of tip in the *smallest units* of the currency (integer, **not** float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed *max_tip_amount*.
    pub suggested_tip_amounts: Option<Vec<i64>>,
    /// *Optional*. A JSON-serialized object for data about the invoice, which will be shared with the payment provider. A detailed description of the required fields should be provided by the payment provider.
    pub provider_data: Option<String>,
    /// *Optional*. URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    pub photo_url: Option<String>,
    /// *Optional*. Photo size in bytes
    pub photo_size: Option<i64>,
    /// *Optional*. Photo width
    pub photo_width: Option<i64>,
    /// *Optional*. Photo height
    pub photo_height: Option<i64>,
    /// *Optional*. Pass :code:`True` if you require the user's full name to complete the order
    pub need_name: Option<bool>,
    /// *Optional*. Pass :code:`True` if you require the user's phone number to complete the order
    pub need_phone_number: Option<bool>,
    /// *Optional*. Pass :code:`True` if you require the user's email address to complete the order
    pub need_email: Option<bool>,
    /// *Optional*. Pass :code:`True` if you require the user's shipping address to complete the order
    pub need_shipping_address: Option<bool>,
    /// *Optional*. Pass :code:`True` if the user's phone number should be sent to provider
    pub send_phone_number_to_provider: Option<bool>,
    /// *Optional*. Pass :code:`True` if the user's email address should be sent to provider
    pub send_email_to_provider: Option<bool>,
    /// *Optional*. Pass :code:`True` if the final price depends on the shipping method
    pub is_flexible: Option<bool>,
}
