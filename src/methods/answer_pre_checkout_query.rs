use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an [`Update`](crate::types::Update) with the field `pre_checkout_query`. Use this method to respond to such pre-checkout queries. On success, `true` is returned. Note: The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent. Use this method to respond to such pre-checkout queries.
/// # Documentation
/// <https://core.telegram.org/bots/api#answerprecheckoutquery>
/// # Note
/// The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent
/// # Returns
/// On success, `true` is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct AnswerPreCheckoutQuery {
    /// Unique identifier for the query to be answered
    pub pre_checkout_query_id: String,
    /// Specify `true` if everything is alright (goods are available, etc.) and the bot is ready to proceed with the order. Use `False` if there are any problems.
    pub ok: bool,
    /// Required if `ok` is `False`. Error message in human readable form that explains the reason for failure to proceed with the checkout (e.g. "Sorry, somebody just bought the last of our amazing black T-shirts while you were busy filling out your payment details. Please choose a different color or garment!"). Telegram will display this message to the user.
    pub error_message: Option<String>,
}

impl AnswerPreCheckoutQuery {
    #[must_use]
    pub fn new(pre_checkout_query_id: impl Into<String>, ok: bool) -> Self {
        Self {
            pre_checkout_query_id: pre_checkout_query_id.into(),
            ok,
            error_message: None,
        }
    }
    #[must_use]
    pub fn pre_checkout_query_id(self, val: impl Into<String>) -> Self {
        Self {
            pre_checkout_query_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn ok(self, val: bool) -> Self {
        Self { ok: val, ..self }
    }

    #[must_use]
    pub fn error_message(self, val: impl Into<String>) -> Self {
        Self {
            error_message: Some(val.into()),
            ..self
        }
    }
}

impl AnswerPreCheckoutQuery {
    #[must_use]
    pub fn error_message_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            error_message: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for AnswerPreCheckoutQuery {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("answerPreCheckoutQuery", self, None)
    }
}

impl AsRef<AnswerPreCheckoutQuery> for AnswerPreCheckoutQuery {
    fn as_ref(&self) -> &Self {
        self
    }
}
