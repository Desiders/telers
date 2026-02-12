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
pub struct CreateChatSubscriptionInviteLink {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Invite link name; 0-32 characters
    pub name: Option<String>,
    /// The number of seconds the subscription will be active for before the next payment. Currently, it must always be 2592000 (30 days).
    pub subscription_period: i64,
    /// The amount of Telegram Stars a user must pay initially and after each subsequent subscription period to be a member of the chat; 1-2500
    pub subscription_price: i64,
}

impl CreateChatSubscriptionInviteLink {
    #[must_use]
    pub fn new(
        chat_id: impl Into<ChatIdKind>,
        subscription_period: i64,
        subscription_price: i64,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            name: None,
            subscription_period,
            subscription_price,
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
    pub fn name(self, val: impl Into<String>) -> Self {
        Self {
            name: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn subscription_period(self, val: i64) -> Self {
        Self {
            subscription_period: val,
            ..self
        }
    }

    #[must_use]
    pub fn subscription_price(self, val: i64) -> Self {
        Self {
            subscription_price: val,
            ..self
        }
    }
}

impl CreateChatSubscriptionInviteLink {
    #[must_use]
    pub fn name_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            name: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for CreateChatSubscriptionInviteLink {
    type Method = Self;
    type Return = ChatInviteLink;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("createChatSubscriptionInviteLink", self, None)
    }
}

impl AsRef<CreateChatSubscriptionInviteLink> for CreateChatSubscriptionInviteLink {
    fn as_ref(&self) -> &Self {
        self
    }
}
