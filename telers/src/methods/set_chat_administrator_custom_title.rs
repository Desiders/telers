use crate::client::Bot;
use serde::Serialize;
/// Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setchatadministratorcustomtitle>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetChatAdministratorCustomTitle {
    /// Unique identifier for the target chat or username of the target supergroup in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// New custom title for the administrator; 0-16 characters, emoji are not allowed
    pub custom_title: Box<str>,
}
impl SetChatAdministratorCustomTitle {
    /// Creates a new `SetChatAdministratorCustomTitle`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup in the format @username
    /// * `user_id` - Unique identifier of the target user
    /// * `custom_title` - New custom title for the administrator; 0-16 characters, emoji are not allowed
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<i64>, T2: Into<Box<str>>>(
        chat_id: T0,
        user_id: T1,
        custom_title: T2,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id: user_id.into(),
            custom_title: custom_title.into(),
        }
    }

    /// Unique identifier for the target chat or username of the target supergroup in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// Unique identifier of the target user
    #[must_use]
    pub fn user_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.user_id = val.into();
        this
    }

    /// New custom title for the administrator; 0-16 characters, emoji are not allowed
    #[must_use]
    pub fn custom_title<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.custom_title = val.into();
        this
    }
}
impl super::TelegramMethod for SetChatAdministratorCustomTitle {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setChatAdministratorCustomTitle", self, None)
    }
}
