use crate::client::Bot;
use serde::Serialize;
/// If you sent an invoice requesting a shipping address and the parameter `is_flexible` was specified, the Bot API will send an Update with a `shipping_query` field to the bot. Use this method to reply to shipping queries. On success, `true` is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#answershippingquery>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct AnswerShippingQuery {
    /// Unique identifier for the query to be answered
    pub shipping_query_id: Box<str>,
    /// Pass `true` if delivery to the specified address is possible and `false` if there are any problems (for example, if delivery to the specified address is not possible)
    pub ok: bool,
    /// Required if ok is `true`. A JSON-serialized Array of available shipping options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Box<[crate::types::ShippingOption]>>,
    /// Required if ok is `false`. Error message in human readable form that explains why it is impossible to complete the order (e.g. `Sorry, delivery to your desired address is unavailable`). Telegram will display this message to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<Box<str>>,
}
impl AnswerShippingQuery {
    /// Creates a new `AnswerShippingQuery`.
    ///
    /// # Arguments
    /// * `shipping_query_id` - Unique identifier for the query to be answered
    /// * `ok` - Pass `true` if delivery to the specified address is possible and `false` if there are any problems (for example, if delivery to the specified address is not possible)
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<bool>>(shipping_query_id: T0, ok: T1) -> Self {
        Self {
            shipping_query_id: shipping_query_id.into(),
            ok: ok.into(),
            shipping_options: None,
            error_message: None,
        }
    }

    /// Unique identifier for the query to be answered
    #[must_use]
    pub fn shipping_query_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.shipping_query_id = val.into();
        self
    }

    /// Pass `true` if delivery to the specified address is possible and `false` if there are any problems (for example, if delivery to the specified address is not possible)
    #[must_use]
    pub fn ok<T: Into<bool>>(mut self, val: T) -> Self {
        self.ok = val.into();
        self
    }

    /// Required if ok is `true`. A JSON-serialized Array of available shipping options.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn shipping_options<
        TItem: Into<crate::types::ShippingOption>,
        T: IntoIterator<Item = TItem>,
    >(
        mut self,
        val: T,
    ) -> Self {
        self.shipping_options = Some(
            self.shipping_options
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        self
    }

    /// Required if ok is `true`. A JSON-serialized Array of available shipping options.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn shipping_option<T: Into<crate::types::ShippingOption>>(mut self, val: T) -> Self {
        self.shipping_options = Some(
            self.shipping_options
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// Required if ok is `true`. A JSON-serialized Array of available shipping options.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn shipping_options_option<
        TItem: Into<crate::types::ShippingOption>,
        T: IntoIterator<Item = TItem>,
    >(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.shipping_options = val.map(|v| v.into_iter().map(Into::into).collect());
        self
    }

    /// Required if ok is `false`. Error message in human readable form that explains why it is impossible to complete the order (e.g. `Sorry, delivery to your desired address is unavailable`). Telegram will display this message to the user.
    #[must_use]
    pub fn error_message<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.error_message = Some(val.into());
        self
    }

    /// Required if ok is `false`. Error message in human readable form that explains why it is impossible to complete the order (e.g. `Sorry, delivery to your desired address is unavailable`). Telegram will display this message to the user.
    #[must_use]
    pub fn error_message_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.error_message = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for AnswerShippingQuery {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("answerShippingQuery", self, None)
    }
}
