use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{InlineQueryResult, PreparedInlineMessage},
};

use serde::Serialize;

/// Stores a message that can be sent by a user of a Mini App
/// # Documentation
/// <https://core.telegram.org/bots/api#savepreparedinlinemessage>
/// # Returns
/// On success, a [`PreparedInlineMessage`] object is returned
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SavePreparedInlineMessage {
    /// Unique identifier of the target user that can use the prepared message
    pub user_id: i64,
    /// A JSON-serialized object describing the message to be sent
    pub result: InlineQueryResult,
    /// `true`, if the message can be sent to private chats with users
    pub allow_user_chats: Option<bool>,
    /// `true`, if the message can be sent to private chats with bots
    pub allow_bot_chats: Option<bool>,
    /// `true`, if the message can be sent to group and supergroup chats
    pub allow_group_chats: Option<bool>,
    /// `true`, if the message can be sent to channel chats
    pub allow_channel_chats: Option<bool>,
}

impl SavePreparedInlineMessage {
    #[must_use]
    pub fn new(user_id: i64, result: impl Into<InlineQueryResult>) -> Self {
        Self {
            user_id,
            result: result.into(),
            allow_user_chats: None,
            allow_bot_chats: None,
            allow_group_chats: None,
            allow_channel_chats: None,
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
    pub fn result(self, val: impl Into<InlineQueryResult>) -> Self {
        Self {
            result: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn allow_user_chats(self, val: bool) -> Self {
        Self {
            allow_user_chats: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn allow_bot_chats(self, val: bool) -> Self {
        Self {
            allow_bot_chats: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn allow_group_chats(self, val: bool) -> Self {
        Self {
            allow_group_chats: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn allow_channel_chats(self, val: bool) -> Self {
        Self {
            allow_channel_chats: Some(val),
            ..self
        }
    }
}

impl SavePreparedInlineMessage {
    #[must_use]
    pub fn allow_user_chats_option(self, val: Option<bool>) -> Self {
        Self {
            allow_user_chats: val,
            ..self
        }
    }

    #[must_use]
    pub fn allow_bot_chats_option(self, val: Option<bool>) -> Self {
        Self {
            allow_bot_chats: val,
            ..self
        }
    }

    #[must_use]
    pub fn allow_group_chats_option(self, val: Option<bool>) -> Self {
        Self {
            allow_group_chats: val,
            ..self
        }
    }

    #[must_use]
    pub fn allow_channel_chats_option(self, val: Option<bool>) -> Self {
        Self {
            allow_channel_chats: val,
            ..self
        }
    }
}

impl TelegramMethod for SavePreparedInlineMessage {
    type Method = Self;
    type Return = PreparedInlineMessage;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("savePreparedInlineMessage", self, None)
    }
}

impl AsRef<SavePreparedInlineMessage> for SavePreparedInlineMessage {
    fn as_ref(&self) -> &Self {
        self
    }
}
