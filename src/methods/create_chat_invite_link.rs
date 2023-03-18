use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, ChatInviteLink},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method [`RevokeChatInviteLink`](crate::methods::RevokeChatInviteLink).
/// # Documentation
/// <https://core.telegram.org/bots/api#createchatinvitelink>
/// # Returns
/// Returns the new invite link as [`ChatInviteLink`] object
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct CreateChatInviteLink {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Invite link name; 0-32 characters
    pub invite_link: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire
    pub expire_date: Option<i64>,
    /// The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    pub member_limit: Option<i64>,
    /// `True`, if users joining the chat via the link need to be approved by chat administrators. If `True`, `member_limit` can't be specified
    pub creates_join_request: Option<bool>,
}

impl CreateChatInviteLink {
    #[must_use]
    pub fn new<T: Into<ChatIdKind>>(chat_id: T) -> Self {
        Self {
            chat_id: chat_id.into(),
            invite_link: None,
            expire_date: None,
            member_limit: None,
            creates_join_request: None,
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    #[must_use]
    pub fn invite_link<T: Into<String>>(mut self, val: T) -> Self {
        self.invite_link = Some(val.into());
        self
    }

    #[must_use]
    pub fn expire_date(mut self, val: i64) -> Self {
        self.expire_date = Some(val);
        self
    }

    #[must_use]
    pub fn member_limit(mut self, val: i64) -> Self {
        self.member_limit = Some(val);
        self
    }

    #[must_use]
    pub fn creates_join_request(mut self, val: bool) -> Self {
        self.creates_join_request = Some(val);
        self
    }
}

impl CreateChatInviteLink {
    #[must_use]
    pub fn invite_link_some<T: Into<String>>(mut self, val: Option<T>) -> Self {
        self.invite_link = val.map(Into::into);
        self
    }

    #[must_use]
    pub fn expire_date_some(mut self, val: Option<i64>) -> Self {
        self.expire_date = val;
        self
    }

    #[must_use]
    pub fn member_limit_some(mut self, val: Option<i64>) -> Self {
        self.member_limit = val;
        self
    }

    #[must_use]
    pub fn creates_join_request_some(mut self, val: Option<bool>) -> Self {
        self.creates_join_request = val;
        self
    }
}

impl TelegramMethod for CreateChatInviteLink {
    type Method = Self;
    type Return = ChatInviteLink;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("createChatInviteLink", self, None)
    }
}
