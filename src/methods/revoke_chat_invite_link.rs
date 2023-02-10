use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, ChatInviteLink},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// # Documentation
/// <https://core.telegram.org/bots/api#revokechatinvitelink>
/// # Returns
/// Returns the revoked invite link as [`ChatInviteLink`] object
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct RevokeChatInviteLink {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// The invite link to revoke
    pub invite_link: String,
}

impl RevokeChatInviteLink {
    #[must_use]
    pub fn new<C: Into<ChatIdKind>, T: Into<String>>(chat_id: C, invite_link: T) -> Self {
        Self {
            chat_id: chat_id.into(),
            invite_link: invite_link.into(),
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    #[must_use]
    pub fn invite_link<T: Into<String>>(mut self, val: T) -> Self {
        self.invite_link = val.into();
        self
    }
}

impl TelegramMethod for RevokeChatInviteLink {
    type Method = Self;
    type Return = ChatInviteLink;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("revokeChatInviteLink", self, None)
    }
}
