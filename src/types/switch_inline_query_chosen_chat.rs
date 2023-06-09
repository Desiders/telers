use serde::{Deserialize, Serialize};

/// This object represents an inline button that switches the current user to inline mode in a chosen chat, with an optional default inline query.
/// # Documentation
/// <https://core.telegram.org/bots/api#switchinlinequerychosenchat>
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SwitchInlineQueryChosenChat {
    /// The default inline query to be inserted in the input field. If left empty, only the bot's username will be inserted
    pub query: Option<String>,
    /// `True`, if private chats with users can be chosen
    pub allow_user_chats: Option<bool>,
    /// `True`, if private chats with bots can be chosen
    pub allow_bot_chats: Option<bool>,
    /// `True`, if group and supergroup chats can be chosen
    pub allow_group_chats: Option<bool>,
    /// `True`, if channel chats can be chosen
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
