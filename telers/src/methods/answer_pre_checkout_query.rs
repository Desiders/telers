use crate::client::Bot;
use serde::Serialize;
/// Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an Update with the field `pre_checkout_query`. Use this method to respond to such pre-checkout queries. On success, `true` is returned. Note: The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
/// # Documentation
/// <https://core.telegram.org/bots/api#answerprecheckoutquery>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct AnswerPreCheckoutQuery {
    /// Unique identifier for the query to be answered
    pub pre_checkout_query_id: Box<str>,
    /// Specify `true` if everything is alright (goods are available, etc.) and the bot is ready to proceed with the order. Use `false` if there are any problems.
    pub ok: bool,
    /// Required if ok is `false`. Error message in human readable form that explains the reason for failure to proceed with the checkout (e.g. `Sorry, somebody just bought the last of our amazing black T-shirts while you were busy filling out your payment details. Please choose a different color or garment!`). Telegram will display this message to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<Box<str>>,
}
impl AnswerPreCheckoutQuery {
    /// Creates a new `AnswerPreCheckoutQuery`.
    ///
    /// # Arguments
    /// * `pre_checkout_query_id` - Unique identifier for the query to be answered
    /// * `ok` - Specify `true` if everything is alright (goods are available, etc.) and the bot is ready to proceed with the order. Use `false` if there are any problems.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<bool>>(pre_checkout_query_id: T0, ok: T1) -> Self {
        Self {
            pre_checkout_query_id: pre_checkout_query_id.into(),
            ok: ok.into(),
            error_message: None,
        }
    }

    /// Unique identifier for the query to be answered
    #[must_use]
    pub fn pre_checkout_query_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.pre_checkout_query_id = val.into();
        self
    }

    /// Specify `true` if everything is alright (goods are available, etc.) and the bot is ready to proceed with the order. Use `false` if there are any problems.
    #[must_use]
    pub fn ok<T: Into<bool>>(mut self, val: T) -> Self {
        self.ok = val.into();
        self
    }

    /// Required if ok is `false`. Error message in human readable form that explains the reason for failure to proceed with the checkout (e.g. `Sorry, somebody just bought the last of our amazing black T-shirts while you were busy filling out your payment details. Please choose a different color or garment!`). Telegram will display this message to the user.
    #[must_use]
    pub fn error_message<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.error_message = Some(val.into());
        self
    }

    /// Required if ok is `false`. Error message in human readable form that explains the reason for failure to proceed with the checkout (e.g. `Sorry, somebody just bought the last of our amazing black T-shirts while you were busy filling out your payment details. Please choose a different color or garment!`). Telegram will display this message to the user.
    #[must_use]
    pub fn error_message_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.error_message = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for AnswerPreCheckoutQuery {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("answerPreCheckoutQuery", self, None)
    }
}
