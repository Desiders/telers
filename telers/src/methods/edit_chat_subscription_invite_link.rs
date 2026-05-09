use crate::client::Bot;
use serde::Serialize;
/// Use this method to edit a subscription invite link created by the bot. The bot must have the `can_invite_users` administrator rights. Returns the edited invite link as a [`crate::types::ChatInviteLink`] object.
/// # Documentation
/// <https://core.telegram.org/bots/api#editchatsubscriptioninvitelink>
/// # Returns
/// - `crate::types::ChatInviteLink`
#[derive(Clone, Debug, Serialize)]
pub struct EditChatSubscriptionInviteLink {
    /// Unique identifier for the target chat or username of the target channel in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// The invite link to edit
    pub invite_link: Box<str>,
    /// Invite link name; 0-32 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<str>>,
}
impl EditChatSubscriptionInviteLink {
    /// Creates a new `EditChatSubscriptionInviteLink`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel in the format @username
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
        }
    }

    /// Unique identifier for the target chat or username of the target channel in the format @username
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
}
impl super::TelegramMethod for EditChatSubscriptionInviteLink {
    type Method = Self;
    type Return = crate::types::ChatInviteLink;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("editChatSubscriptionInviteLink", self, None)
    }
}
