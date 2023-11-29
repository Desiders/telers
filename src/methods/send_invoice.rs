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
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
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
    /// Pass `true` if you require the user's full name to complete the order
    pub need_name: Option<bool>,
    /// Pass `true` if you require the user's phone number to complete the order
    pub need_phone_number: Option<bool>,
    /// Pass `true` if you require the user's email address to complete the order
    pub need_email: Option<bool>,
    /// Pass `true` if you require the user's shipping address to complete the order
    pub need_shipping_address: Option<bool>,
    /// Pass `true` if the user's phone number should be sent to provider
    pub send_phone_number_to_provider: Option<bool>,
    /// Pass `true` if the user's email address should be sent to provider
    pub send_email_to_provider: Option<bool>,
    /// Pass `true` if the final price depends on the shipping method
    pub is_flexible: Option<bool>,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// Pass `true`, if the message should be sent even if the specified replied-to message is not found
    pub allow_sending_without_reply: Option<bool>,
    /// A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards). If empty, one 'Pay `total price`' button will be shown. If not empty, the first button must be a Pay button.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl SendInvoice {
    #[must_use]
    pub fn new(
        chat_id: impl Into<ChatIdKind>,
        title: impl Into<String>,
        description: impl Into<String>,
        payload: impl Into<String>,
        provider_token: impl Into<String>,
        currency: impl Into<String>,
        prices: impl IntoIterator<Item = LabeledPrice>,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: None,
            title: title.into(),
            description: description.into(),
            payload: payload.into(),
            provider_token: provider_token.into(),
            currency: currency.into(),
            prices: prices.into_iter().collect(),
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
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn message_thread_id(self, val: i64) -> Self {
        Self {
            message_thread_id: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn title(self, val: impl Into<String>) -> Self {
        Self {
            title: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn description(self, val: impl Into<String>) -> Self {
        Self {
            description: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn payload(self, val: impl Into<String>) -> Self {
        Self {
            payload: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn provider_token(self, val: impl Into<String>) -> Self {
        Self {
            provider_token: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn currency(self, val: impl Into<String>) -> Self {
        Self {
            currency: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn price(self, val: LabeledPrice) -> Self {
        Self {
            prices: self.prices.into_iter().chain(Some(val)).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn prices(self, val: impl IntoIterator<Item = LabeledPrice>) -> Self {
        Self {
            prices: self.prices.into_iter().chain(val).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn suggested_tip_amount(self, val: i64) -> Self {
        Self {
            suggested_tip_amounts: Some(
                self.suggested_tip_amounts
                    .unwrap_or_default()
                    .into_iter()
                    .chain(Some(val))
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn suggested_tip_amounts(self, val: impl IntoIterator<Item = i64>) -> Self {
        Self {
            suggested_tip_amounts: Some(
                self.suggested_tip_amounts
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn start_parameter(self, val: impl Into<String>) -> Self {
        Self {
            start_parameter: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn provider_data(self, val: impl Into<String>) -> Self {
        Self {
            provider_data: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn photo_url(self, val: impl Into<String>) -> Self {
        Self {
            photo_url: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn photo_size(self, val: i64) -> Self {
        Self {
            photo_size: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn photo_width(self, val: i64) -> Self {
        Self {
            photo_width: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn photo_height(self, val: i64) -> Self {
        Self {
            photo_height: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn need_name(self, val: bool) -> Self {
        Self {
            need_name: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn need_phone_number(self, val: bool) -> Self {
        Self {
            need_phone_number: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn need_email(self, val: bool) -> Self {
        Self {
            need_email: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn need_shipping_address(self, val: bool) -> Self {
        Self {
            need_shipping_address: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn send_phone_number_to_provider(self, val: bool) -> Self {
        Self {
            send_phone_number_to_provider: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn send_email_to_provider(self, val: bool) -> Self {
        Self {
            send_email_to_provider: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn is_flexible(self, val: bool) -> Self {
        Self {
            is_flexible: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn disable_notification(self, val: bool) -> Self {
        Self {
            disable_notification: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn protect_content(self, val: bool) -> Self {
        Self {
            protect_content: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn reply_to_message_id(self, val: i64) -> Self {
        Self {
            reply_to_message_id: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn allow_sending_without_reply(self, val: bool) -> Self {
        Self {
            allow_sending_without_reply: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup(self, val: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(val.into()),
            ..self
        }
    }
}

impl SendInvoice {
    #[must_use]
    pub fn max_tip_amount_option(self, val: Option<i64>) -> Self {
        Self {
            max_tip_amount: val,
            ..self
        }
    }

    #[must_use]
    pub fn suggested_tip_amounts_option(self, val: Option<impl IntoIterator<Item = i64>>) -> Self {
        Self {
            suggested_tip_amounts: val.map(|val| val.into_iter().collect()),
            ..self
        }
    }

    #[must_use]
    pub fn start_parameter_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            start_parameter: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn provider_data_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            provider_data: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn photo_url_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            photo_url: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn photo_size_option(self, val: Option<i64>) -> Self {
        Self {
            photo_size: val,
            ..self
        }
    }

    #[must_use]
    pub fn photo_width_option(self, val: Option<i64>) -> Self {
        Self {
            photo_width: val,
            ..self
        }
    }

    #[must_use]
    pub fn photo_height_option(self, val: Option<i64>) -> Self {
        Self {
            photo_height: val,
            ..self
        }
    }

    #[must_use]
    pub fn need_name_option(self, val: Option<bool>) -> Self {
        Self {
            need_name: val,
            ..self
        }
    }

    #[must_use]
    pub fn need_phone_number_option(self, val: Option<bool>) -> Self {
        Self {
            need_phone_number: val,
            ..self
        }
    }

    #[must_use]
    pub fn need_email_option(self, val: Option<bool>) -> Self {
        Self {
            need_email: val,
            ..self
        }
    }

    #[must_use]
    pub fn need_shipping_address_option(self, val: Option<bool>) -> Self {
        Self {
            need_shipping_address: val,
            ..self
        }
    }

    #[must_use]
    pub fn send_phone_number_to_provider_option(self, val: Option<bool>) -> Self {
        Self {
            send_phone_number_to_provider: val,
            ..self
        }
    }

    #[must_use]
    pub fn send_email_to_provider_option(self, val: Option<bool>) -> Self {
        Self {
            send_email_to_provider: val,
            ..self
        }
    }

    #[must_use]
    pub fn is_flexible_option(self, val: Option<bool>) -> Self {
        Self {
            is_flexible: val,
            ..self
        }
    }

    #[must_use]
    pub fn disable_notification_option(self, val: Option<bool>) -> Self {
        Self {
            disable_notification: val,
            ..self
        }
    }

    #[must_use]
    pub fn protect_content_option(self, val: Option<bool>) -> Self {
        Self {
            protect_content: val,
            ..self
        }
    }

    #[must_use]
    pub fn reply_to_message_id_option(self, val: Option<i64>) -> Self {
        Self {
            reply_to_message_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn allow_sending_without_reply_option(self, val: Option<bool>) -> Self {
        Self {
            allow_sending_without_reply: val,
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup_option(self, val: Option<impl Into<InlineKeyboardMarkup>>) -> Self {
        Self {
            reply_markup: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for SendInvoice {
    type Method = Self;
    type Return = Message;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("sendInvoice", self, None)
    }
}

impl AsRef<SendInvoice> for SendInvoice {
    fn as_ref(&self) -> &Self {
        self
    }
}
