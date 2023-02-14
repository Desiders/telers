use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ShippingOption};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// If you sent an invoice requesting a shipping address and the parameter `is_flexible` was specified, the Bot API will send an [`Update`](crate::types::Update) with a `shipping_query` field to the bot. Use this method to reply to shipping queries.
/// # Documentation
/// <https://core.telegram.org/bots/api#answershippingquery>
/// # Returns
/// On success, `True` is returned
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct AnswerShippingQuery {
    /// Unique identifier for the query to be answered
    pub shipping_query_id: String,
    /// Pass `True` if delivery to the specified address is possible and `False` if there are any problems (for example, if delivery to the specified address is not possible)
    pub ok: bool,
    /// Required if `ok` is `True`. A JSON-serialized array of available shipping options.
    pub shipping_options: Option<Vec<ShippingOption>>,
    /// Required if `ok` is `False`. Error message in human readable form that explains why it is impossible to complete the order (e.g. "Sorry, delivery to your desired address is unavailable'). Telegram will display this message to the user.
    pub error_message: Option<String>,
}

impl AnswerShippingQuery {
    #[must_use]
    pub fn new<T: Into<String>>(shipping_query_id: T, ok: bool) -> Self {
        Self {
            shipping_query_id: shipping_query_id.into(),
            ok,
            shipping_options: None,
            error_message: None,
        }
    }

    #[must_use]
    pub fn shipping_query_id<T: Into<String>>(mut self, val: T) -> Self {
        self.shipping_query_id = val.into();
        self
    }

    #[must_use]
    pub fn ok(mut self, val: bool) -> Self {
        self.ok = val;
        self
    }

    #[must_use]
    pub fn shipping_options(mut self, val: Vec<ShippingOption>) -> Self {
        self.shipping_options = Some(val);
        self
    }

    #[must_use]
    pub fn error_message<T: Into<String>>(mut self, val: T) -> Self {
        self.error_message = Some(val.into());
        self
    }
}

impl TelegramMethod for AnswerShippingQuery {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("answerShippingQuery", self, None)
    }
}
