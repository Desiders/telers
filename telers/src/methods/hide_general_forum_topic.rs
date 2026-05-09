use crate::client::Bot;
use serde::Serialize;
/// Use this method to hide the 'General' topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the `can_manage_topics` administrator rights. The topic will be automatically closed if it was open. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#hidegeneralforumtopic>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct HideGeneralForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup in the format @username
    pub chat_id: crate::types::ChatIdKind,
}
impl HideGeneralForumTopic {
    /// Creates a new `HideGeneralForumTopic`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup in the format @username
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>>(chat_id: T0) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }

    /// Unique identifier for the target chat or username of the target supergroup in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }
}
impl super::TelegramMethod for HideGeneralForumTopic {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("hideGeneralForumTopic", self, None)
    }
}
