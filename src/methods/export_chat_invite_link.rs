use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights.
/// # Documentation
/// <https://core.telegram.org/bots/api#exportchatinvitelink>
/// # Notes
/// Each administrator in a chat generates their own invite links. Bots can't use invite links generated by other administrators. If you want your bot to work with invite links, it will need to generate its own link using [`crate::methods::ExportChatInviteLink`] or by calling the [`crate::methods::GetChat`] method. If your bot needs to generate a new primary invite link replacing its previous one, use [`crate::methods::ExportChatInviteLink`] again.
/// # Returns
/// Returns the new invite link as `String` on success
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct ExportChatInviteLink {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
}

impl ExportChatInviteLink {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: val.into(),
        }
    }
}

impl TelegramMethod for ExportChatInviteLink {
    type Method = Self;
    type Return = String;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("exportChatInviteLink", self, None)
    }
}

impl AsRef<ExportChatInviteLink> for ExportChatInviteLink {
    fn as_ref(&self) -> &Self {
        self
    }
}
