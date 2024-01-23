use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, ChatInviteLink},
};

use serde::Serialize;

/// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// # Documentation
/// <https://core.telegram.org/bots/api#revokechatinvitelink>
/// # Returns
/// Returns the revoked invite link as [`ChatInviteLink`] object
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct RevokeChatInviteLink {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// The invite link to revoke
    pub invite_link: String,
}

impl RevokeChatInviteLink {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, invite_link: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            invite_link: invite_link.into(),
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn invite_link(self, val: impl Into<String>) -> Self {
        Self {
            invite_link: val.into(),
            ..self
        }
    }
}

impl TelegramMethod for RevokeChatInviteLink {
    type Method = Self;
    type Return = ChatInviteLink;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("revokeChatInviteLink", self, None)
    }
}

impl AsRef<RevokeChatInviteLink> for RevokeChatInviteLink {
    fn as_ref(&self) -> &Self {
        self
    }
}
