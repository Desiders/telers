use crate::client::Bot;
use serde::Serialize;
/// Use this method to send invoices. On success, the sent Message is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendinvoice>
/// # Returns
/// - `crate::types::Message`
#[derive(Clone, Debug, Serialize)]
pub struct SendInvoice {
    /// Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
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
    /// A JSON-serialized array of suggested amounts of tips in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed `max_tip_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Box<[i64]>>,
    /// Unique deep-linking parameter. If left empty, forwarded copies of the sent message will have a Pay button, allowing multiple users to pay directly from the forwarded message, using the same invoice. If non-empty, forwarded copies of the sent message will have a URL button with a deep link to the bot (instead of a Pay button), with the value used as the start parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<Box<str>>,
    /// JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<Box<str>>,
    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for.
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
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass `true` to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<Box<str>>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<crate::types::SuggestedPostParameters>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<crate::types::ReplyParameters>,
    /// A JSON-serialized object for an inline keyboard. If empty, one 'Pay total price' button will be shown. If not empty, the first button must be a Pay button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
}
impl SendInvoice {
    /// Creates a new `SendInvoice`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username
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
        T0: Into<crate::types::ChatIdKind>,
        T1: Into<Box<str>>,
        T2: Into<Box<str>>,
        T3: Into<Box<str>>,
        T4: Into<Box<str>>,
        T5Item: Into<crate::types::LabeledPrice>,
        T5: IntoIterator<Item = T5Item>,
    >(
        chat_id: T0,
        title: T1,
        description: T2,
        payload: T3,
        currency: T4,
        prices: T5,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: None,
            direct_messages_topic_id: None,
            title: title.into(),
            description: description.into(),
            payload: payload.into(),
            provider_token: None,
            currency: currency.into(),
            prices: prices.into_iter().map(Into::into).collect(),
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
            allow_paid_broadcast: None,
            message_effect_id: None,
            suggested_post_parameters: None,
            reply_parameters: None,
            reply_markup: None,
        }
    }

    /// Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[must_use]
    pub fn message_thread_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.message_thread_id = Some(val.into());
        this
    }

    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[must_use]
    pub fn message_thread_id_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.message_thread_id = val.map(Into::into);
        this
    }

    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[must_use]
    pub fn direct_messages_topic_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.direct_messages_topic_id = Some(val.into());
        this
    }

    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[must_use]
    pub fn direct_messages_topic_id_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.direct_messages_topic_id = val.map(Into::into);
        this
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
    pub fn prices<TItem: Into<crate::types::LabeledPrice>, T: IntoIterator<Item = TItem>>(
        self,
        val: T,
    ) -> Self {
        let mut this = self;
        this.prices = this
            .prices
            .into_vec()
            .into_iter()
            .chain(val.into_iter().map(Into::into))
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

    /// A JSON-serialized array of suggested amounts of tips in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed `max_tip_amount`.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn suggested_tip_amounts<TItem: Into<i64>, T: IntoIterator<Item = TItem>>(
        self,
        val: T,
    ) -> Self {
        let mut this = self;
        this.suggested_tip_amounts = Some(
            this.suggested_tip_amounts
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        this
    }

    /// A JSON-serialized array of suggested amounts of tips in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed `max_tip_amount`.
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

    /// A JSON-serialized array of suggested amounts of tips in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed `max_tip_amount`.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn suggested_tip_amounts_option<TItem: Into<i64>, T: IntoIterator<Item = TItem>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.suggested_tip_amounts = val.map(|v| v.into_iter().map(Into::into).collect());
        this
    }

    /// Unique deep-linking parameter. If left empty, forwarded copies of the sent message will have a Pay button, allowing multiple users to pay directly from the forwarded message, using the same invoice. If non-empty, forwarded copies of the sent message will have a URL button with a deep link to the bot (instead of a Pay button), with the value used as the start parameter
    #[must_use]
    pub fn start_parameter<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.start_parameter = Some(val.into());
        this
    }

    /// Unique deep-linking parameter. If left empty, forwarded copies of the sent message will have a Pay button, allowing multiple users to pay directly from the forwarded message, using the same invoice. If non-empty, forwarded copies of the sent message will have a URL button with a deep link to the bot (instead of a Pay button), with the value used as the start parameter
    #[must_use]
    pub fn start_parameter_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.start_parameter = val.map(Into::into);
        this
    }

    /// JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    #[must_use]
    pub fn provider_data<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.provider_data = Some(val.into());
        this
    }

    /// JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
    #[must_use]
    pub fn provider_data_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.provider_data = val.map(Into::into);
        this
    }

    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for.
    #[must_use]
    pub fn photo_url<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.photo_url = Some(val.into());
        this
    }

    /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for.
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

    /// Sends the message silently. Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.disable_notification = Some(val.into());
        this
    }

    /// Sends the message silently. Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.disable_notification = val.map(Into::into);
        this
    }

    /// Protects the contents of the sent message from forwarding and saving
    #[must_use]
    pub fn protect_content<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.protect_content = Some(val.into());
        this
    }

    /// Protects the contents of the sent message from forwarding and saving
    #[must_use]
    pub fn protect_content_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.protect_content = val.map(Into::into);
        this
    }

    /// Pass `true` to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance.
    #[must_use]
    pub fn allow_paid_broadcast<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.allow_paid_broadcast = Some(val.into());
        this
    }

    /// Pass `true` to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance.
    #[must_use]
    pub fn allow_paid_broadcast_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.allow_paid_broadcast = val.map(Into::into);
        this
    }

    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[must_use]
    pub fn message_effect_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.message_effect_id = Some(val.into());
        this
    }

    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[must_use]
    pub fn message_effect_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.message_effect_id = val.map(Into::into);
        this
    }

    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[must_use]
    pub fn suggested_post_parameters<T: Into<crate::types::SuggestedPostParameters>>(
        self,
        val: T,
    ) -> Self {
        let mut this = self;
        this.suggested_post_parameters = Some(val.into());
        this
    }

    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[must_use]
    pub fn suggested_post_parameters_option<T: Into<crate::types::SuggestedPostParameters>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.suggested_post_parameters = val.map(Into::into);
        this
    }

    /// Description of the message to reply to
    #[must_use]
    pub fn reply_parameters<T: Into<crate::types::ReplyParameters>>(self, val: T) -> Self {
        let mut this = self;
        this.reply_parameters = Some(val.into());
        this
    }

    /// Description of the message to reply to
    #[must_use]
    pub fn reply_parameters_option<T: Into<crate::types::ReplyParameters>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.reply_parameters = val.map(Into::into);
        this
    }

    /// A JSON-serialized object for an inline keyboard. If empty, one 'Pay total price' button will be shown. If not empty, the first button must be a Pay button.
    #[must_use]
    pub fn reply_markup<T: Into<crate::types::InlineKeyboardMarkup>>(self, val: T) -> Self {
        let mut this = self;
        this.reply_markup = Some(val.into());
        this
    }

    /// A JSON-serialized object for an inline keyboard. If empty, one 'Pay total price' button will be shown. If not empty, the first button must be a Pay button.
    #[must_use]
    pub fn reply_markup_option<T: Into<crate::types::InlineKeyboardMarkup>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.reply_markup = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for SendInvoice {
    type Method = Self;
    type Return = crate::types::Message;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("sendInvoice", self, None)
    }
}
