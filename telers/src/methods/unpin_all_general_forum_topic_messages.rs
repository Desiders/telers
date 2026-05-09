use crate::client::Bot;
use serde::Serialize;
/// Use this method to clear the list of pinned messages in a General forum topic. The bot must be an administrator in the chat for this to work and must have the `can_pin_messages` administrator right in the supergroup. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#unpinallgeneralforumtopicmessages>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct UnpinAllGeneralForumTopicMessages {
    /// Unique identifier for the target chat or username of the target supergroup in the format @username
    pub chat_id: crate::types::ChatIdKind,
}
impl UnpinAllGeneralForumTopicMessages {
    /// Creates a new `UnpinAllGeneralForumTopicMessages`.
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
impl super::TelegramMethod for UnpinAllGeneralForumTopicMessages {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("unpinAllGeneralForumTopicMessages", self, None)
    }
}
