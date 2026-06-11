use crate::client::Bot;
use serde::Serialize;
/// Use this method to process a received chat join request query. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#answerchatjoinrequestquery>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct AnswerChatJoinRequestQuery {
    /// Unique identifier of the join request query
    pub chat_join_request_query_id: Box<str>,
    /// Result of the query. Must be either `approve` to allow the user to join the chat, `decline` to disallow the user to join the chat, or `queue` to leave the decision to other administrators.
    pub result: Box<str>,
}
impl AnswerChatJoinRequestQuery {
    /// Creates a new `AnswerChatJoinRequestQuery`.
    ///
    /// # Arguments
    /// * `chat_join_request_query_id` - Unique identifier of the join request query
    /// * `result` - Result of the query. Must be either `approve` to allow the user to join the chat, `decline` to disallow the user to join the chat, or `queue` to leave the decision to other administrators.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(
        chat_join_request_query_id: T0,
        result: T1,
    ) -> Self {
        Self {
            chat_join_request_query_id: chat_join_request_query_id.into(),
            result: result.into(),
        }
    }

    /// Unique identifier of the join request query
    #[must_use]
    pub fn chat_join_request_query_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.chat_join_request_query_id = val.into();
        self
    }

    /// Result of the query. Must be either `approve` to allow the user to join the chat, `decline` to disallow the user to join the chat, or `queue` to leave the decision to other administrators.
    #[must_use]
    pub fn result<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.result = val.into();
        self
    }
}
impl super::TelegramMethod for AnswerChatJoinRequestQuery {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("answerChatJoinRequestQuery", self, None)
    }
}
