use crate::client::Bot;
use serde::Serialize;
/// Use this method to edit the name of the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the `can_manage_topics` administrator rights. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#editgeneralforumtopic>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct EditGeneralForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// New topic name, 1-128 characters
    pub name: Box<str>,
}
impl EditGeneralForumTopic {
    /// Creates a new `EditGeneralForumTopic`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup in the format @username
    /// * `name` - New topic name, 1-128 characters
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<Box<str>>>(
        chat_id: T0,
        name: T1,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            name: name.into(),
        }
    }

    /// Unique identifier for the target chat or username of the target supergroup in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// New topic name, 1-128 characters
    #[must_use]
    pub fn name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.name = val.into();
        this
    }
}
impl super::TelegramMethod for EditGeneralForumTopic {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("editGeneralForumTopic", self, None)
    }
}
