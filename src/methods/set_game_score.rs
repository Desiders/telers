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
    pub fn user_id(self, val: i64) -> Self {
        Self {
            user_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn score(self, val: u64) -> Self {
        Self { score: val, ..self }
    }

    #[must_use]
    pub fn force(self, val: bool) -> Self {
        Self {
            force: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn disable_edit_message(self, val: bool) -> Self {
        Self {
            disable_edit_message: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn chat_id(self, val: i64) -> Self {
        Self {
            chat_id: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn message_id(self, val: i64) -> Self {
        Self {
            message_id: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn inline_message_id(self, val: impl Into<String>) -> Self {
        Self {
            inline_message_id: Some(val.into()),
            ..self
        }
    }
}

impl SetGameScore {
    #[must_use]
    pub fn force_option(self, val: Option<bool>) -> Self {
        Self { force: val, ..self }
    }

    #[must_use]
    pub fn disable_edit_message_option(self, val: Option<bool>) -> Self {
        Self {
            disable_edit_message: val,
            ..self
        }
    }

    #[must_use]
    pub fn chat_id_option(self, val: Option<i64>) -> Self {
        Self {
            chat_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn message_id_option(self, val: Option<i64>) -> Self {
        Self {
            message_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn inline_message_id_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            inline_message_id: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for SetGameScore {
    type Method = Self;
    type Return = MessageOrTrue;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setGameScore", self, None)
    }
}

impl AsRef<SetGameScore> for SetGameScore {
    fn as_ref(&self) -> &Self {
        self
    }
}
