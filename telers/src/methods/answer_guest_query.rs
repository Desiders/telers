use crate::client::Bot;
use serde::Serialize;
/// Use this method to reply to a received guest message. On success, a [`crate::types::SentGuestMessage`] object is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#answerguestquery>
/// # Returns
/// - `crate::types::SentGuestMessage`
#[derive(Clone, Debug, Serialize)]
pub struct AnswerGuestQuery {
    /// Unique identifier for the query to be answered
    pub guest_query_id: Box<str>,
    /// A JSON-serialized object describing the message to be sent
    pub result: crate::types::InlineQueryResult,
}
impl AnswerGuestQuery {
    /// Creates a new `AnswerGuestQuery`.
    ///
    /// # Arguments
    /// * `guest_query_id` - Unique identifier for the query to be answered
    /// * `result` - A JSON-serialized object describing the message to be sent
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<crate::types::InlineQueryResult>>(
        guest_query_id: T0,
        result: T1,
    ) -> Self {
        Self {
            guest_query_id: guest_query_id.into(),
            result: result.into(),
        }
    }

    /// Unique identifier for the query to be answered
    #[must_use]
    pub fn guest_query_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.guest_query_id = val.into();
        self
    }

    /// A JSON-serialized object describing the message to be sent
    #[must_use]
    pub fn result<T: Into<crate::types::InlineQueryResult>>(mut self, val: T) -> Self {
        self.result = val.into();
        self
    }
}
impl super::TelegramMethod for AnswerGuestQuery {
    type Method = Self;
    type Return = crate::types::SentGuestMessage;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("answerGuestQuery", self, None)
    }
}
