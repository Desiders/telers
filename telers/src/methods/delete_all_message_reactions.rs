use crate::client::Bot;
use serde::Serialize;
/// Use this method to remove up to 10000 recent reactions in a group or a supergroup chat added by a given user or chat. The bot must have the '`can_delete_messages`' administrator right in the chat. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#deleteallmessagereactions>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct DeleteAllMessageReactions {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @username)
    pub chat_id: crate::types::ChatIdKind,
    /// Identifier of the user whose reactions will be removed, if the reactions were added by a user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Identifier of the chat whose reactions will be removed, if the reactions were added by a chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_chat_id: Option<i64>,
}
impl DeleteAllMessageReactions {
    /// Creates a new `DeleteAllMessageReactions`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup (in the format @username)
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>>(chat_id: T0) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id: None,
            actor_chat_id: None,
        }
    }

    /// Unique identifier for the target chat or username of the target supergroup (in the format @username)
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// Identifier of the user whose reactions will be removed, if the reactions were added by a user
    #[must_use]
    pub fn user_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.user_id = Some(val.into());
        this
    }

    /// Identifier of the user whose reactions will be removed, if the reactions were added by a user
    #[must_use]
    pub fn user_id_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.user_id = val.map(Into::into);
        this
    }

    /// Identifier of the chat whose reactions will be removed, if the reactions were added by a chat
    #[must_use]
    pub fn actor_chat_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.actor_chat_id = Some(val.into());
        this
    }

    /// Identifier of the chat whose reactions will be removed, if the reactions were added by a chat
    #[must_use]
    pub fn actor_chat_id_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.actor_chat_id = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for DeleteAllMessageReactions {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("deleteAllMessageReactions", self, None)
    }
}
