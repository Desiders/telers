use crate::client::Bot;
use serde::Serialize;
/// Use this method to approve a suggested post in a direct messages chat. The bot must have the '`can_post_messages`' administrator right in the corresponding channel chat. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#approvesuggestedpost>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct ApproveSuggestedPost {
    /// Unique identifier for the target direct messages chat
    pub chat_id: i64,
    /// Identifier of a suggested post message to approve
    pub message_id: i64,
    /// Point in time (Unix timestamp) when the post is expected to be published; omit if the date has already been specified when the suggested post was created. If specified, then the date must be not more than 2678400 seconds (30 days) in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_date: Option<i64>,
}
impl ApproveSuggestedPost {
    /// Creates a new `ApproveSuggestedPost`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target direct messages chat
    /// * `message_id` - Identifier of a suggested post message to approve
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>>(chat_id: T0, message_id: T1) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id: message_id.into(),
            send_date: None,
        }
    }

    /// Unique identifier for the target direct messages chat
    #[must_use]
    pub fn chat_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    /// Identifier of a suggested post message to approve
    #[must_use]
    pub fn message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_id = val.into();
        self
    }

    /// Point in time (Unix timestamp) when the post is expected to be published; omit if the date has already been specified when the suggested post was created. If specified, then the date must be not more than 2678400 seconds (30 days) in the future.
    #[must_use]
    pub fn send_date<T: Into<i64>>(mut self, val: T) -> Self {
        self.send_date = Some(val.into());
        self
    }

    /// Point in time (Unix timestamp) when the post is expected to be published; omit if the date has already been specified when the suggested post was created. If specified, then the date must be not more than 2678400 seconds (30 days) in the future.
    #[must_use]
    pub fn send_date_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.send_date = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for ApproveSuggestedPost {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("approveSuggestedPost", self, None)
    }
}
