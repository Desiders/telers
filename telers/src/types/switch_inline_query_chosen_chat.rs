use serde::{Deserialize, Serialize};

/// This object represents an inline button that switches the current user to inline mode in a chosen chat, with an optional default inline query.
/// # Documentation
/// <https://core.telegram.org/bots/api#switchinlinequerychosenchat>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct SwitchInlineQueryChosenChat {
    /// The default inline query to be inserted in the input field. If left empty, only the bot's username will be inserted
    pub query: Option<String>,
    /// `true`, if private chats with users can be chosen
    pub allow_user_chats: Option<bool>,
    /// `true`, if private chats with bots can be chosen
    pub allow_bot_chats: Option<bool>,
    /// `true`, if group and supergroup chats can be chosen
    pub allow_group_chats: Option<bool>,
    /// `true`, if channel chats can be chosen
    pub allow_channel_chats: Option<bool>,
}

impl SwitchInlineQueryChosenChat {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn query(self, val: impl Into<String>) -> Self {
        Self {
            query: Some(val.into()),
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

impl SwitchInlineQueryChosenChat {
    #[must_use]
    pub fn query_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            query: val.map(Into::into),
            ..self
        }
    }

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
