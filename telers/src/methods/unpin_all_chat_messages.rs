use crate::client::Bot;
use serde::Serialize;
/// Use this method to clear the list of pinned messages in a chat. In private chats and channel direct messages chats, no additional rights are required to unpin all pinned messages. Conversely, the bot must be an administrator with the '`can_pin_messages`' right or the '`can_edit_messages`' right to unpin all pinned messages in groups and channels respectively. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#unpinallchatmessages>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct UnpinAllChatMessages {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: crate::types::ChatIdKind,
}
impl UnpinAllChatMessages {
    /// Creates a new `UnpinAllChatMessages`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>>(chat_id: T0) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }

    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }
}
impl super::TelegramMethod for UnpinAllChatMessages {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("unpinAllChatMessages", self, None)
    }
}
