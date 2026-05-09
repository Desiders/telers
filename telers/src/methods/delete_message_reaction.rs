use crate::client::Bot;
use serde::Serialize;
/// Use this method to remove a reaction from a message in a group or a supergroup chat. The bot must have the '`can_delete_messages`' administrator right in the chat. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#deletemessagereaction>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct DeleteMessageReaction {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @username)
    pub chat_id: crate::types::ChatIdKind,
    /// Identifier of the target message
    pub message_id: i64,
    /// Identifier of the user whose reaction will be removed, if the reaction was added by a user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Identifier of the chat whose reaction will be removed, if the reaction was added by a chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_chat_id: Option<i64>,
}
impl DeleteMessageReaction {
    /// Creates a new `DeleteMessageReaction`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup (in the format @username)
    /// * `message_id` - Identifier of the target message
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<i64>>(
        chat_id: T0,
        message_id: T1,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id: message_id.into(),
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

    /// Identifier of the target message
    #[must_use]
    pub fn message_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.message_id = val.into();
        this
    }

    /// Identifier of the user whose reaction will be removed, if the reaction was added by a user
    #[must_use]
    pub fn user_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.user_id = Some(val.into());
        this
    }

    /// Identifier of the user whose reaction will be removed, if the reaction was added by a user
    #[must_use]
    pub fn user_id_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.user_id = val.map(Into::into);
        this
    }

    /// Identifier of the chat whose reaction will be removed, if the reaction was added by a chat
    #[must_use]
    pub fn actor_chat_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.actor_chat_id = Some(val.into());
        this
    }

    /// Identifier of the chat whose reaction will be removed, if the reaction was added by a chat
    #[must_use]
    pub fn actor_chat_id_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.actor_chat_id = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for DeleteMessageReaction {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("deleteMessageReaction", self, None)
    }
}
