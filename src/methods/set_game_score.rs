use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::MessageOrTrue};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to send a game
/// # Documentation
/// <https://core.telegram.org/bots/api#setgamescore>
/// # Returns
/// On success, the sent [`MessageOrTrue`] is returned
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SetGameScore {
    /// User identifier
    pub user_id: i64,
    /// New score, must be non-negative
    pub score: u64,
    /// Pass `True`, if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters
    pub force: Option<bool>,
    /// Pass `True`, if the game message should not be automatically edited to include the current scoreboard
    pub disable_edit_message: Option<bool>,
    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat
    pub chat_id: Option<i64>,
    /// Required if `inline_message_id` is not specified. Identifier of the sent message
    pub message_id: Option<i64>,
    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
}

impl SetGameScore {
    #[must_use]
    pub fn new(user_id: i64, score: u64) -> Self {
        Self {
            user_id,
            score,
            force: None,
            disable_edit_message: None,
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
    pub fn score(mut self, val: u64) -> Self {
        self.score = val;
        self
    }

    #[must_use]
    pub fn force(mut self, val: bool) -> Self {
        self.force = Some(val);
        self
    }

    #[must_use]
    pub fn disable_edit_message(mut self, val: bool) -> Self {
        self.disable_edit_message = Some(val);
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

impl TelegramMethod for SetGameScore {
    type Method = Self;
    type Return = MessageOrTrue;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setGameScore", self, None)
    }
}
