use serde::{Deserialize, Serialize};
/// This object defines the criteria used to request a suitable chat. Information about the selected chat will be shared with the bot when the corresponding button is pressed. The bot will be granted requested rights in the chat if appropriate. More about requesting chats: <https://core.telegram.org/bots/features#chat-and-user-selection>.
/// # Documentation
/// <https://core.telegram.org/bots/api#keyboardbuttonrequestchat>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyboardButtonRequestChat {
    /// Signed 32-bit identifier of the request, which will be received back in the [`crate::types::ChatShared`] object. Must be unique within the message.
    pub request_id: i64,
    /// Pass `true` to request a channel chat, pass `false` to request a group or a supergroup chat
    pub chat_is_channel: bool,
    /// Pass `true` to request a forum supergroup, pass `false` to request a non-forum chat. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_forum: Option<bool>,
    /// Pass `true` to request a supergroup or a channel with a username, pass `false` to request a chat without a username. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_has_username: Option<bool>,
    /// Pass `true` to request a chat owned by the user. Otherwise, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_created: Option<bool>,
    /// A JSON-serialized object listing the required administrator rights of the user in the chat. The rights must be a superset of `bot_administrator_rights`. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_administrator_rights: Option<crate::types::ChatAdministratorRights>,
    /// A JSON-serialized object listing the required administrator rights of the bot in the chat. The rights must be a subset of `user_administrator_rights`. If not specified, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_administrator_rights: Option<crate::types::ChatAdministratorRights>,
    /// Pass `true` to request a chat with the bot as a member. Otherwise, no additional restrictions are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_is_member: Option<bool>,
    /// Pass `true` to request the chat's title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_title: Option<bool>,
    /// Pass `true` to request the chat's username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,
    /// Pass `true` to request the chat's photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}
impl KeyboardButtonRequestChat {
    /// Creates a new `KeyboardButtonRequestChat`.
    ///
    /// # Arguments
    /// * `request_id` - Signed 32-bit identifier of the request, which will be received back in the [`crate::types::ChatShared`] object. Must be unique within the message.
    /// * `chat_is_channel` - Pass `true` to request a channel chat, pass `false` to request a group or a supergroup chat
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<bool>>(request_id: T0, chat_is_channel: T1) -> Self {
        Self {
            request_id: request_id.into(),
            chat_is_channel: chat_is_channel.into(),
            chat_is_forum: None,
            chat_has_username: None,
            chat_is_created: None,
            user_administrator_rights: None,
            bot_administrator_rights: None,
            bot_is_member: None,
            request_title: None,
            request_username: None,
            request_photo: None,
        }
    }

    /// Signed 32-bit identifier of the request, which will be received back in the [`crate::types::ChatShared`] object. Must be unique within the message.
    #[must_use]
    pub fn request_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.request_id = val.into();
        self
    }

    /// Pass `true` to request a channel chat, pass `false` to request a group or a supergroup chat
    #[must_use]
    pub fn chat_is_channel<T: Into<bool>>(mut self, val: T) -> Self {
        self.chat_is_channel = val.into();
        self
    }

    /// Pass `true` to request a forum supergroup, pass `false` to request a non-forum chat. If not specified, no additional restrictions are applied.
    #[must_use]
    pub fn chat_is_forum<T: Into<bool>>(mut self, val: T) -> Self {
        self.chat_is_forum = Some(val.into());
        self
    }

    /// Pass `true` to request a forum supergroup, pass `false` to request a non-forum chat. If not specified, no additional restrictions are applied.
    #[must_use]
    pub fn chat_is_forum_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.chat_is_forum = val.map(Into::into);
        self
    }

    /// Pass `true` to request a supergroup or a channel with a username, pass `false` to request a chat without a username. If not specified, no additional restrictions are applied.
    #[must_use]
    pub fn chat_has_username<T: Into<bool>>(mut self, val: T) -> Self {
        self.chat_has_username = Some(val.into());
        self
    }

    /// Pass `true` to request a supergroup or a channel with a username, pass `false` to request a chat without a username. If not specified, no additional restrictions are applied.
    #[must_use]
    pub fn chat_has_username_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.chat_has_username = val.map(Into::into);
        self
    }

    /// Pass `true` to request a chat owned by the user. Otherwise, no additional restrictions are applied.
    #[must_use]
    pub fn chat_is_created<T: Into<bool>>(mut self, val: T) -> Self {
        self.chat_is_created = Some(val.into());
        self
    }

    /// Pass `true` to request a chat owned by the user. Otherwise, no additional restrictions are applied.
    #[must_use]
    pub fn chat_is_created_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.chat_is_created = val.map(Into::into);
        self
    }

    /// A JSON-serialized object listing the required administrator rights of the user in the chat. The rights must be a superset of `bot_administrator_rights`. If not specified, no additional restrictions are applied.
    #[must_use]
    pub fn user_administrator_rights<T: Into<crate::types::ChatAdministratorRights>>(
        mut self,
        val: T,
    ) -> Self {
        self.user_administrator_rights = Some(val.into());
        self
    }

    /// A JSON-serialized object listing the required administrator rights of the user in the chat. The rights must be a superset of `bot_administrator_rights`. If not specified, no additional restrictions are applied.
    #[must_use]
    pub fn user_administrator_rights_option<T: Into<crate::types::ChatAdministratorRights>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.user_administrator_rights = val.map(Into::into);
        self
    }

    /// A JSON-serialized object listing the required administrator rights of the bot in the chat. The rights must be a subset of `user_administrator_rights`. If not specified, no additional restrictions are applied.
    #[must_use]
    pub fn bot_administrator_rights<T: Into<crate::types::ChatAdministratorRights>>(
        mut self,
        val: T,
    ) -> Self {
        self.bot_administrator_rights = Some(val.into());
        self
    }

    /// A JSON-serialized object listing the required administrator rights of the bot in the chat. The rights must be a subset of `user_administrator_rights`. If not specified, no additional restrictions are applied.
    #[must_use]
    pub fn bot_administrator_rights_option<T: Into<crate::types::ChatAdministratorRights>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.bot_administrator_rights = val.map(Into::into);
        self
    }

    /// Pass `true` to request a chat with the bot as a member. Otherwise, no additional restrictions are applied.
    #[must_use]
    pub fn bot_is_member<T: Into<bool>>(mut self, val: T) -> Self {
        self.bot_is_member = Some(val.into());
        self
    }

    /// Pass `true` to request a chat with the bot as a member. Otherwise, no additional restrictions are applied.
    #[must_use]
    pub fn bot_is_member_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.bot_is_member = val.map(Into::into);
        self
    }

    /// Pass `true` to request the chat's title
    #[must_use]
    pub fn request_title<T: Into<bool>>(mut self, val: T) -> Self {
        self.request_title = Some(val.into());
        self
    }

    /// Pass `true` to request the chat's title
    #[must_use]
    pub fn request_title_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.request_title = val.map(Into::into);
        self
    }

    /// Pass `true` to request the chat's username
    #[must_use]
    pub fn request_username<T: Into<bool>>(mut self, val: T) -> Self {
        self.request_username = Some(val.into());
        self
    }

    /// Pass `true` to request the chat's username
    #[must_use]
    pub fn request_username_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.request_username = val.map(Into::into);
        self
    }

    /// Pass `true` to request the chat's photo
    #[must_use]
    pub fn request_photo<T: Into<bool>>(mut self, val: T) -> Self {
        self.request_photo = Some(val.into());
        self
    }

    /// Pass `true` to request the chat's photo
    #[must_use]
    pub fn request_photo_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.request_photo = val.map(Into::into);
        self
    }
}
