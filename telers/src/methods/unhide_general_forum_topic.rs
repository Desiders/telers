use crate::client::Bot;
use serde::Serialize;
/// Use this method to unhide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the `can_manage_topics` administrator rights. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#unhidegeneralforumtopic>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct UnhideGeneralForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: crate::types::ChatIdKind,
}
impl UnhideGeneralForumTopic {
    /// Creates a new `UnhideGeneralForumTopic`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>>(chat_id: T0) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }

    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }
}
impl super::TelegramMethod for UnhideGeneralForumTopic {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("unhideGeneralForumTopic", self, None)
    }
}
