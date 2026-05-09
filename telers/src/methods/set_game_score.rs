use crate::client::Bot;
use serde::Serialize;
/// Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the Message is returned, otherwise `true` is returned. Returns an error, if the new score is not greater than the user's current score in the chat and force is `false`.
/// # Documentation
/// <https://core.telegram.org/bots/api#setgamescore>
/// # Returns
/// - `crate::types::Message`
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetGameScore {
    /// User identifier
    pub user_id: i64,
    /// New score, must be non-negative
    pub score: i64,
    /// Pass `true` if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// Pass `true` if the game message should not be automatically edited to include the current scoreboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_edit_message: Option<bool>,
    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if `inline_message_id` is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<Box<str>>,
}
impl SetGameScore {
    /// Creates a new `SetGameScore`.
    ///
    /// # Arguments
    /// * `user_id` - User identifier
    /// * `score` - New score, must be non-negative
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>>(user_id: T0, score: T1) -> Self {
        Self {
            user_id: user_id.into(),
            score: score.into(),
            force: None,
            disable_edit_message: None,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
        }
    }

    /// User identifier
    #[must_use]
    pub fn user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.user_id = val.into();
        self
    }

    /// New score, must be non-negative
    #[must_use]
    pub fn score<T: Into<i64>>(mut self, val: T) -> Self {
        self.score = val.into();
        self
    }

    /// Pass `true` if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters
    #[must_use]
    pub fn force<T: Into<bool>>(mut self, val: T) -> Self {
        self.force = Some(val.into());
        self
    }

    /// Pass `true` if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters
    #[must_use]
    pub fn force_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.force = val.map(Into::into);
        self
    }

    /// Pass `true` if the game message should not be automatically edited to include the current scoreboard
    #[must_use]
    pub fn disable_edit_message<T: Into<bool>>(mut self, val: T) -> Self {
        self.disable_edit_message = Some(val.into());
        self
    }

    /// Pass `true` if the game message should not be automatically edited to include the current scoreboard
    #[must_use]
    pub fn disable_edit_message_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.disable_edit_message = val.map(Into::into);
        self
    }

    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat
    #[must_use]
    pub fn chat_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.chat_id = Some(val.into());
        self
    }

    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat
    #[must_use]
    pub fn chat_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.chat_id = val.map(Into::into);
        self
    }

    /// Required if `inline_message_id` is not specified. Identifier of the sent message
    #[must_use]
    pub fn message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_id = Some(val.into());
        self
    }

    /// Required if `inline_message_id` is not specified. Identifier of the sent message
    #[must_use]
    pub fn message_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.message_id = val.map(Into::into);
        self
    }

    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message
    #[must_use]
    pub fn inline_message_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.inline_message_id = Some(val.into());
        self
    }

    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message
    #[must_use]
    pub fn inline_message_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.inline_message_id = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for SetGameScore {
    type Method = Self;
    type Return = crate::Either<crate::types::Message, bool>;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setGameScore", self, None)
    }
}
