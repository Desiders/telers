use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to clear the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the `can_pin_messages` administrator right in a supergroup or `can_edit_messages` administrator right in a channel.
/// # Documentation
/// <https://core.telegram.org/bots/api#unpinallchatmessages>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct UnpinAllChatMessages {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
}

impl UnpinAllChatMessages {
    #[must_use]
    pub fn new<T: Into<ChatIdKind>>(chat_id: T) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }
}

impl TelegramMethod for UnpinAllChatMessages {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("unpinAllChatMessages", self, None)
    }
}
