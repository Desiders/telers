use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, ChatInviteLink},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to create a [`subscription invite link`](https://telegram.org/blog/superchannels-star-reactions-subscriptions#star-subscriptions) for a channel chat. The bot must have the `can_invite_users` administrator rights. The link can be edited using the method [`EditChatSubscriptionInviteLink`](super::EditChatSubscriptionInviteLink) or revoked using the method [`RevokeChatInviteLink`](super::RevokeChatInviteLink).
/// # Documentation
/// <https://core.telegram.org/bots/api#createchatsubscriptioninvitelink>
/// # Returns
/// Returns the new invite link as a [`ChatInviteLink`] object
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct EditChatSubscriptionInviteLink {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// The invite link to edit
    pub invite_link: String,
    /// Invite link name; 0-32 characters
    pub name: Option<String>,
}

impl EditChatSubscriptionInviteLink {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, invite_link: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            invite_link: invite_link.into(),
            name: None,
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

    #[must_use]
    pub fn name(self, val: impl Into<String>) -> Self {
        Self {
            name: Some(val.into()),
            ..self
        }
    }
}

impl EditChatSubscriptionInviteLink {
    #[must_use]
    pub fn name_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            name: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for EditChatSubscriptionInviteLink {
    type Method = Self;
    type Return = ChatInviteLink;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<'_, Self::Method> {
        Request::new("createChatSubscriptionInviteLink", self, None)
    }
}

impl AsRef<EditChatSubscriptionInviteLink> for EditChatSubscriptionInviteLink {
    fn as_ref(&self) -> &Self {
        self
    }
}
