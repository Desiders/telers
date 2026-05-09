use serde::{Deserialize, Serialize};
/// This object represents an inline button that switches the current user to inline mode in a chosen chat, with an optional default inline query.
/// # Documentation
/// <https://core.telegram.org/bots/api#switchinlinequerychosenchat>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SwitchInlineQueryChosenChat {
    /// The default inline query to be inserted in the input field. If left empty, only the bot's username will be inserted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Box<str>>,
    /// `true`, if private chats with users can be chosen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_chats: Option<bool>,
    /// `true`, if private chats with bots can be chosen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_bot_chats: Option<bool>,
    /// `true`, if group and supergroup chats can be chosen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_group_chats: Option<bool>,
    /// `true`, if channel chats can be chosen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_channel_chats: Option<bool>,
}
impl SwitchInlineQueryChosenChat {
    /// Creates a new `SwitchInlineQueryChosenChat`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            query: None,
            allow_user_chats: None,
            allow_bot_chats: None,
            allow_group_chats: None,
            allow_channel_chats: None,
        }
    }

    /// The default inline query to be inserted in the input field. If left empty, only the bot's username will be inserted
    #[must_use]
    pub fn query<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.query = Some(val.into());
        self
    }

    /// The default inline query to be inserted in the input field. If left empty, only the bot's username will be inserted
    #[must_use]
    pub fn query_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.query = val.map(Into::into);
        self
    }

    /// `true`, if private chats with users can be chosen
    #[must_use]
    pub fn allow_user_chats<T: Into<bool>>(mut self, val: T) -> Self {
        self.allow_user_chats = Some(val.into());
        self
    }

    /// `true`, if private chats with users can be chosen
    #[must_use]
    pub fn allow_user_chats_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.allow_user_chats = val.map(Into::into);
        self
    }

    /// `true`, if private chats with bots can be chosen
    #[must_use]
    pub fn allow_bot_chats<T: Into<bool>>(mut self, val: T) -> Self {
        self.allow_bot_chats = Some(val.into());
        self
    }

    /// `true`, if private chats with bots can be chosen
    #[must_use]
    pub fn allow_bot_chats_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.allow_bot_chats = val.map(Into::into);
        self
    }

    /// `true`, if group and supergroup chats can be chosen
    #[must_use]
    pub fn allow_group_chats<T: Into<bool>>(mut self, val: T) -> Self {
        self.allow_group_chats = Some(val.into());
        self
    }

    /// `true`, if group and supergroup chats can be chosen
    #[must_use]
    pub fn allow_group_chats_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.allow_group_chats = val.map(Into::into);
        self
    }

    /// `true`, if channel chats can be chosen
    #[must_use]
    pub fn allow_channel_chats<T: Into<bool>>(mut self, val: T) -> Self {
        self.allow_channel_chats = Some(val.into());
        self
    }

    /// `true`, if channel chats can be chosen
    #[must_use]
    pub fn allow_channel_chats_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.allow_channel_chats = val.map(Into::into);
        self
    }
}
impl Default for SwitchInlineQueryChosenChat {
    fn default() -> Self {
        Self::new()
    }
}
