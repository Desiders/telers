use crate::client::Bot;
use serde::Serialize;
/// Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game. Returns an Array of [`crate::types::GameHighScore`] objects.
/// # Documentation
/// <https://core.telegram.org/bots/api#getgamehighscores>
/// # Returns
/// - `Box<[crate::types::GameHighScore]>`
#[derive(Clone, Debug, Serialize)]
pub struct GetGameHighScores {
    /// Target user id
    pub user_id: i64,
    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if `inline_message_id` is not specified. Identifier of the sent message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<Box<str>>,
}
impl GetGameHighScores {
    /// Creates a new `GetGameHighScores`.
    ///
    /// # Arguments
    /// * `user_id` - Target user id
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>>(user_id: T0) -> Self {
        Self {
            user_id: user_id.into(),
            chat_id: None,
            message_id: None,
            inline_message_id: None,
        }
    }

    /// Target user id
    #[must_use]
    pub fn user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.user_id = val.into();
        self
    }

    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat.
    #[must_use]
    pub fn chat_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.chat_id = Some(val.into());
        self
    }

    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat.
    #[must_use]
    pub fn chat_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.chat_id = val.map(Into::into);
        self
    }

    /// Required if `inline_message_id` is not specified. Identifier of the sent message.
    #[must_use]
    pub fn message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_id = Some(val.into());
        self
    }

    /// Required if `inline_message_id` is not specified. Identifier of the sent message.
    #[must_use]
    pub fn message_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.message_id = val.map(Into::into);
        self
    }

    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message.
    #[must_use]
    pub fn inline_message_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.inline_message_id = Some(val.into());
        self
    }

    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message.
    #[must_use]
    pub fn inline_message_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.inline_message_id = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for GetGameHighScores {
    type Method = Self;
    type Return = Box<[crate::types::GameHighScore]>;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getGameHighScores", self, None)
    }
}
