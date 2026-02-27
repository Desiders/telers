use crate::client::Bot;
use serde::Serialize;
/// Stores a message that can be sent by a user of a Mini App. Returns a [`PreparedInlineMessage`] object.
/// # Documentation
/// <https://core.telegram.org/bots/api#savepreparedinlinemessage>
/// # Returns
/// - `crate::types::PreparedInlineMessage`
#[derive(Clone, Debug, Serialize)]
pub struct SavePreparedInlineMessage {
    /// Unique identifier of the target user that can use the prepared message
    pub user_id: i64,
    /// A JSON-serialized object describing the message to be sent
    pub result: crate::types::InlineQueryResult,
    /// Pass `true` if the message can be sent to private chats with users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_chats: Option<bool>,
    /// Pass `true` if the message can be sent to private chats with bots
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_bot_chats: Option<bool>,
    /// Pass `true` if the message can be sent to group and supergroup chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_group_chats: Option<bool>,
    /// Pass `true` if the message can be sent to channel chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_channel_chats: Option<bool>,
}
impl SavePreparedInlineMessage {
    /// Creates a new `SavePreparedInlineMessage`.
    ///
    /// # Arguments
    /// * `user_id` - Unique identifier of the target user that can use the prepared message
    /// * `result` - A JSON-serialized object describing the message to be sent
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::InlineQueryResult>>(
        user_id: T0,
        result: T1,
    ) -> Self {
        Self {
            user_id: user_id.into(),
            result: result.into(),
            allow_user_chats: None,
            allow_bot_chats: None,
            allow_group_chats: None,
            allow_channel_chats: None,
        }
    }

    /// Unique identifier of the target user that can use the prepared message
    #[must_use]
    pub fn user_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.user_id = val.into();
        this
    }

    /// A JSON-serialized object describing the message to be sent
    #[must_use]
    pub fn result<T: Into<crate::types::InlineQueryResult>>(self, val: T) -> Self {
        let mut this = self;
        this.result = val.into();
        this
    }

    /// Pass `true` if the message can be sent to private chats with users
    #[must_use]
    pub fn allow_user_chats<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.allow_user_chats = Some(val.into());
        this
    }

    /// Pass `true` if the message can be sent to private chats with users
    #[must_use]
    pub fn allow_user_chats_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.allow_user_chats = val.map(Into::into);
        this
    }

    /// Pass `true` if the message can be sent to private chats with bots
    #[must_use]
    pub fn allow_bot_chats<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.allow_bot_chats = Some(val.into());
        this
    }

    /// Pass `true` if the message can be sent to private chats with bots
    #[must_use]
    pub fn allow_bot_chats_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.allow_bot_chats = val.map(Into::into);
        this
    }

    /// Pass `true` if the message can be sent to group and supergroup chats
    #[must_use]
    pub fn allow_group_chats<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.allow_group_chats = Some(val.into());
        this
    }

    /// Pass `true` if the message can be sent to group and supergroup chats
    #[must_use]
    pub fn allow_group_chats_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.allow_group_chats = val.map(Into::into);
        this
    }

    /// Pass `true` if the message can be sent to channel chats
    #[must_use]
    pub fn allow_channel_chats<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.allow_channel_chats = Some(val.into());
        this
    }

    /// Pass `true` if the message can be sent to channel chats
    #[must_use]
    pub fn allow_channel_chats_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.allow_channel_chats = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for SavePreparedInlineMessage {
    type Method = Self;
    type Return = crate::types::PreparedInlineMessage;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("savePreparedInlineMessage", self, None)
    }
}
