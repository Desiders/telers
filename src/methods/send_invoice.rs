use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, InlineKeyboardMarkup, LabeledPrice, Message},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to send invoices
/// # Documentation
/// <https://core.telegram.org/bots/api#sendinvoice>
/// # Returns
/// On success, the sent [`Message`] is returned
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SendInvoice {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    pub message_thread_id: Option<i64>,
    /// Product name, 1-32 characters
    pub title: String,
    /// Product description, 1-255 characters
    pub description: String,
    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for your internal processes.
    pub payload: String,
    /// Payments provider token, obtained via [`Botfather`](https://t.me/botfather)
    pub provider_token: String,
    /// Three-letter ISO 4217 currency code, see [`more on currencies`](https://core.telegram.org/bots/payments#supported-currencies)
    pub currency: String,
    /// Price breakdown, a list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)
    pub prices: Vec<LabeledPrice>,
    /// The maximum accepted amount for tips in the *smallest units* of the currency (integer, **not** float/double). For example, for a maximum tip of `US$ 1.45` pass `max_tip_amount = 145`. See the *exp* parameter in [`currencies.json`](https://core.telegram.org/bots/payments/currencies.json), it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0
    pub max_tip_amount: Option<i64>,
    /// A JSON-serialized array of suggested amounts of tip in the *smallest units* of the currency (integer, **not** float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed `max_tip_amount`.
    pub suggested_tip_amounts: Option<Vec<i64>>,
    /// Unique deep-linking parameter. If left empty, **forwarded copies** of the sent message will have a Pay button, allowing multiple users to pay directly from the forwarded message, using the same invoice. If non-empty, forwarded copies of the sent message will have a URL button with a deep link to the bot (instead of a Pay button), with the value used as the start parameter.
    pub start_parameter: Option<String>,
    /// A JSON-serialized object for data about the invoice, which will be shared with the payment provider. A detailed description of the required fields should be provided by the payment provider.
    pub provider_data: Option<String>,
    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    pub photo_url: Option<String>,
    /// Photo size in bytes
    pub photo_size: Option<i64>,
    /// Photo width
    pub photo_width: Option<i64>,
    /// Photo height
    pub photo_height: Option<i64>,
    /// Pass `True` if you require the user's full name to complete the order
    pub need_name: Option<bool>,
    /// Pass `True` if you require the user's phone number to complete the order
    pub need_phone_number: Option<bool>,
    /// Pass `True` if you require the user's email address to complete the order
    pub need_email: Option<bool>,
    /// Pass `True` if you require the user's shipping address to complete the order
    pub need_shipping_address: Option<bool>,
    /// Pass `True` if the user's phone number should be sent to provider
    pub send_phone_number_to_provider: Option<bool>,
    /// Pass `True` if the user's email address should be sent to provider
    pub send_email_to_provider: Option<bool>,
    /// Pass `True` if the final price depends on the shipping method
    pub is_flexible: Option<bool>,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// Pass `True`, if the message should be sent even if the specified replied-to message is not found
    pub allow_sending_without_reply: Option<bool>,
    /// A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards). If empty, one 'Pay `total price`' button will be shown. If not empty, the first button must be a Pay button.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl SendInvoice {
    #[must_use]
    pub fn new<C: Into<ChatIdKind>, S: Into<String>>(
        chat_id: C,
        title: S,
        description: S,
        payload: S,
        provider_token: S,
        currency: S,
        prices: Vec<LabeledPrice>,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: None,
            title: title.into(),
            description: description.into(),
            payload: payload.into(),
            provider_token: provider_token.into(),
            currency: currency.into(),
            prices,
            max_tip_amount: None,
            suggested_tip_amounts: None,
            start_parameter: None,
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
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    #[must_use]
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }

    #[must_use]
    pub fn title<T: Into<String>>(mut self, val: T) -> Self {
        self.title = val.into();
        self
    }

    #[must_use]
    pub fn description<T: Into<String>>(mut self, val: T) -> Self {
        self.description = val.into();
        self
    }

    #[must_use]
    pub fn payload<T: Into<String>>(mut self, val: T) -> Self {
        self.payload = val.into();
        self
    }

    #[must_use]
    pub fn provider_token<T: Into<String>>(mut self, val: T) -> Self {
        self.provider_token = val.into();
        self
    }

    #[must_use]
    pub fn currency<T: Into<String>>(mut self, val: T) -> Self {
        self.currency = val.into();
        self
    }

    #[must_use]
    pub fn prices(mut self, val: Vec<LabeledPrice>) -> Self {
        self.prices = val;
        self
    }

    #[must_use]
    pub fn max_tip_amount(mut self, val: i64) -> Self {
        self.max_tip_amount = Some(val);
        self
    }

    #[must_use]
    pub fn suggested_tip_amounts(mut self, val: Vec<i64>) -> Self {
        self.suggested_tip_amounts = Some(val);
        self
    }

    #[must_use]
    pub fn start_parameter<T: Into<String>>(mut self, val: T) -> Self {
        self.start_parameter = Some(val.into());
        self
    }

    #[must_use]
    pub fn provider_data<T: Into<String>>(mut self, val: T) -> Self {
        self.provider_data = Some(val.into());
        self
    }

    #[must_use]
    pub fn photo_url<T: Into<String>>(mut self, val: T) -> Self {
        self.photo_url = Some(val.into());
        self
    }

    #[must_use]
    pub fn photo_size(mut self, val: i64) -> Self {
        self.photo_size = Some(val);
        self
    }

    #[must_use]
    pub fn photo_width(mut self, val: i64) -> Self {
        self.photo_width = Some(val);
        self
    }

    #[must_use]
    pub fn photo_height(mut self, val: i64) -> Self {
        self.photo_height = Some(val);
        self
    }

    #[must_use]
    pub fn need_name(mut self, val: bool) -> Self {
        self.need_name = Some(val);
        self
    }

    #[must_use]
    pub fn need_phone_number(mut self, val: bool) -> Self {
        self.need_phone_number = Some(val);
        self
    }

    #[must_use]
    pub fn need_email(mut self, val: bool) -> Self {
        self.need_email = Some(val);
        self
    }

    #[must_use]
    pub fn need_shipping_address(mut self, val: bool) -> Self {
        self.need_shipping_address = Some(val);
        self
    }

    #[must_use]
    pub fn send_phone_number_to_provider(mut self, val: bool) -> Self {
        self.send_phone_number_to_provider = Some(val);
        self
    }

    #[must_use]
    pub fn send_email_to_provider(mut self, val: bool) -> Self {
        self.send_email_to_provider = Some(val);
        self
    }

    #[must_use]
    pub fn is_flexible(mut self, val: bool) -> Self {
        self.is_flexible = Some(val);
        self
    }

    #[must_use]
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    #[must_use]
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }

    #[must_use]
    pub fn reply_to_message_id(mut self, val: i64) -> Self {
        self.reply_to_message_id = Some(val);
        self
    }

    #[must_use]
    pub fn allow_sending_without_reply(mut self, val: bool) -> Self {
        self.allow_sending_without_reply = Some(val);
        self
    }

    #[must_use]
    pub fn reply_markup<T: Into<InlineKeyboardMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }
}

impl TelegramMethod for SendInvoice {
    type Method = Self;
    type Return = Message;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("sendInvoice", self, None)
    }
}
