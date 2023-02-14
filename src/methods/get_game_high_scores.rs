use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::GameHighScore};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to get data for high score tables. Will return the score of the specified user and several of their neighbors in a game.
/// # Documentation
/// <https://core.telegram.org/bots/api#getgamehighscores>
/// # Note
/// This method will currently return scores for the target user, plus two of their closest neighbors on each side. Will also return the top three users if the user and their neighbors are not among them. Please note that this behavior is subject to change.
/// # Returns
/// Returns an Array of [`GameHighScore`] objects
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct GetGameHighScores {
    /// Target user id
    pub user_id: i64,
    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat
    pub chat_id: Option<i64>,
    /// Required if `inline_message_id` is not specified. Identifier of the sent message
    pub message_id: Option<i64>,
    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
}

impl GetGameHighScores {
    #[must_use]
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
        }
    }

    #[must_use]
    pub fn user_id(mut self, val: i64) -> Self {
        self.user_id = val;
        self
    }

    #[must_use]
    pub fn chat_id(mut self, val: i64) -> Self {
        self.chat_id = Some(val);
        self
    }

    #[must_use]
    pub fn message_id(mut self, val: i64) -> Self {
        self.message_id = Some(val);
        self
    }

    #[must_use]
    pub fn inline_message_id<T: Into<String>>(mut self, val: T) -> Self {
        self.inline_message_id = Some(val.into());
        self
    }
}

impl TelegramMethod for GetGameHighScores {
    type Method = Self;
    type Return = Vec<GameHighScore>;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("getGameHighScores", self, None)
    }
}
