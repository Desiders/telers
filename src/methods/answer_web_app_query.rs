use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{InlineQueryResult, SentWebAppMessage},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to set the result of an interaction with a [`Web App`](https://core.telegram.org/bots/webapps) and send a corresponding message on behalf of the user to the chat from which the query originated.
/// # Documentation
/// <https://core.telegram.org/bots/api#answerwebappquery>
/// # Returns
/// On success, a [`SentWebAppMessage`] object is returned
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AnswerWebAppQuery {
    /// Unique identifier for the query to be answered
    pub web_app_query_id: String,
    /// A JSON-serialized object describing the message to be sent
    pub result: InlineQueryResult,
}

impl AnswerWebAppQuery {
    #[must_use]
    pub fn new(web_app_query_id: impl Into<String>, result: impl Into<InlineQueryResult>) -> Self {
        Self {
            web_app_query_id: web_app_query_id.into(),
            result: result.into(),
        }
    }

    #[must_use]
    pub fn web_app_query_id(self, val: impl Into<String>) -> Self {
        Self {
            web_app_query_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn result(self, val: impl Into<InlineQueryResult>) -> Self {
        Self {
            result: val.into(),
            ..self
        }
    }
}

impl TelegramMethod for AnswerWebAppQuery {
    type Method = Self;
    type Return = SentWebAppMessage;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("answerWebAppQuery", self, None)
    }
}

impl AsRef<AnswerWebAppQuery> for AnswerWebAppQuery {
    fn as_ref(&self) -> &Self {
        self
    }
}
