use crate::client::Bot;
use serde::Serialize;
/// Use this method to decline a suggested post in a direct messages chat. The bot must have the '`can_manage_direct_messages`' administrator right in the corresponding channel chat. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#declinesuggestedpost>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct DeclineSuggestedPost {
    /// Unique identifier for the target direct messages chat
    pub chat_id: i64,
    /// Identifier of a suggested post message to decline
    pub message_id: i64,
    /// Comment for the creator of the suggested post; 0-128 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Box<str>>,
}
impl DeclineSuggestedPost {
    /// Creates a new `DeclineSuggestedPost`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target direct messages chat
    /// * `message_id` - Identifier of a suggested post message to decline
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>>(chat_id: T0, message_id: T1) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id: message_id.into(),
            comment: None,
        }
    }

    /// Unique identifier for the target direct messages chat
    #[must_use]
    pub fn chat_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// Identifier of a suggested post message to decline
    #[must_use]
    pub fn message_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.message_id = val.into();
        this
    }

    /// Comment for the creator of the suggested post; 0-128 characters
    #[must_use]
    pub fn comment<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.comment = Some(val.into());
        this
    }

    /// Comment for the creator of the suggested post; 0-128 characters
    #[must_use]
    pub fn comment_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.comment = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for DeclineSuggestedPost {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("declineSuggestedPost", self, None)
    }
}
