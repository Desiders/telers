use crate::client::Bot;
use serde::Serialize;
/// Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a [`crate::types::ChatInviteLink`] object.
/// # Documentation
/// <https://core.telegram.org/bots/api#editchatinvitelink>
/// # Returns
/// - `crate::types::ChatInviteLink`
#[derive(Clone, Debug, Serialize)]
pub struct EditChatInviteLink {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: crate::types::ChatIdKind,
    /// The invite link to edit
    pub invite_link: Box<str>,
    /// Invite link name; 0-32 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<str>>,
    /// Point in time (Unix timestamp) when the link will expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    /// The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<u32>,
    /// `true`, if users joining the chat via the link need to be approved by chat administrators. If `true`, `member_limit` can't be specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}
impl EditChatInviteLink {
    /// Creates a new `EditChatInviteLink`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    /// * `invite_link` - The invite link to edit
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<Box<str>>>(
        chat_id: T0,
        invite_link: T1,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            invite_link: invite_link.into(),
            name: None,
            expire_date: None,
            member_limit: None,
            creates_join_request: None,
        }
    }

    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// The invite link to edit
    #[must_use]
    pub fn invite_link<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.invite_link = val.into();
        this
    }

    /// Invite link name; 0-32 characters
    #[must_use]
    pub fn name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.name = Some(val.into());
        this
    }

    /// Invite link name; 0-32 characters
    #[must_use]
    pub fn name_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.name = val.map(Into::into);
        this
    }

    /// Point in time (Unix timestamp) when the link will expire
    #[must_use]
    pub fn expire_date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.expire_date = Some(val.into());
        this
    }

    /// Point in time (Unix timestamp) when the link will expire
    #[must_use]
    pub fn expire_date_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.expire_date = val.map(Into::into);
        this
    }

    /// The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[must_use]
    pub fn member_limit<T: Into<u32>>(self, val: T) -> Self {
        let mut this = self;
        this.member_limit = Some(val.into());
        this
    }

    /// The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[must_use]
    pub fn member_limit_option<T: Into<u32>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.member_limit = val.map(Into::into);
        this
    }

    /// `true`, if users joining the chat via the link need to be approved by chat administrators. If `true`, `member_limit` can't be specified
    #[must_use]
    pub fn creates_join_request<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.creates_join_request = Some(val.into());
        this
    }

    /// `true`, if users joining the chat via the link need to be approved by chat administrators. If `true`, `member_limit` can't be specified
    #[must_use]
    pub fn creates_join_request_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.creates_join_request = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for EditChatInviteLink {
    type Method = Self;
    type Return = crate::types::ChatInviteLink;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("editChatInviteLink", self, None)
    }
}
