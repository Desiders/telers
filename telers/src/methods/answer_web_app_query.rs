use crate::client::Bot;
use serde::Serialize;
/// Use this method to set the result of an interaction with a Web App and send a corresponding message on behalf of the user to the chat from which the query originated. On success, a [`SentWebAppMessage`] object is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#answerwebappquery>
/// # Returns
/// - `crate::types::SentWebAppMessage`
#[derive(Clone, Debug, Serialize)]
pub struct AnswerWebAppQuery {
    /// Unique identifier for the query to be answered
    pub web_app_query_id: Box<str>,
    /// A JSON-serialized object describing the message to be sent
    pub result: crate::types::InlineQueryResult,
}
impl AnswerWebAppQuery {
    /// Creates a new `AnswerWebAppQuery`.
    ///
    /// # Arguments
    /// * `web_app_query_id` - Unique identifier for the query to be answered
    /// * `result` - A JSON-serialized object describing the message to be sent
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<crate::types::InlineQueryResult>>(
        web_app_query_id: T0,
        result: T1,
    ) -> Self {
        Self {
            web_app_query_id: web_app_query_id.into(),
            result: result.into(),
        }
    }

    /// Unique identifier for the query to be answered
    #[must_use]
    pub fn web_app_query_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.web_app_query_id = val.into();
        this
    }

    /// A JSON-serialized object describing the message to be sent
    #[must_use]
    pub fn result<T: Into<crate::types::InlineQueryResult>>(self, val: T) -> Self {
        let mut this = self;
        this.result = val.into();
        this
    }
}
impl super::TelegramMethod for AnswerWebAppQuery {
    type Method = Self;
    type Return = crate::types::SentWebAppMessage;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("answerWebAppQuery", self, None)
    }
}
