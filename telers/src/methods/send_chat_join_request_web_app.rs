use crate::client::Bot;
use serde::Serialize;
/// Use this method to process a received chat join request query by showing a Mini App to the user before deciding the outcome. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendchatjoinrequestwebapp>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SendChatJoinRequestWebApp {
    /// Unique identifier of the join request query
    pub chat_join_request_query_id: Box<str>,
    /// The URL of the Mini App to be opened
    pub web_app_url: Box<str>,
}
impl SendChatJoinRequestWebApp {
    /// Creates a new `SendChatJoinRequestWebApp`.
    ///
    /// # Arguments
    /// * `chat_join_request_query_id` - Unique identifier of the join request query
    /// * `web_app_url` - The URL of the Mini App to be opened
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(
        chat_join_request_query_id: T0,
        web_app_url: T1,
    ) -> Self {
        Self {
            chat_join_request_query_id: chat_join_request_query_id.into(),
            web_app_url: web_app_url.into(),
        }
    }

    /// Unique identifier of the join request query
    #[must_use]
    pub fn chat_join_request_query_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.chat_join_request_query_id = val.into();
        self
    }

    /// The URL of the Mini App to be opened
    #[must_use]
    pub fn web_app_url<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.web_app_url = val.into();
        self
    }
}
impl super::TelegramMethod for SendChatJoinRequestWebApp {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("sendChatJoinRequestWebApp", self, None)
    }
}
