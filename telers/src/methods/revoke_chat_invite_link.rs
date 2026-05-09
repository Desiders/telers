use crate::client::Bot;
use serde::Serialize;
/// Use this method to revoke an invite link created by the bot. If the primary link is revoked, a new link is automatically generated. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the revoked invite link as [`crate::types::ChatInviteLink`] object.
/// # Documentation
/// <https://core.telegram.org/bots/api#revokechatinvitelink>
/// # Returns
/// - `crate::types::ChatInviteLink`
#[derive(Clone, Debug, Serialize)]
pub struct RevokeChatInviteLink {
    /// Unique identifier of the target chat or username of the target channel in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// The invite link to revoke
    pub invite_link: Box<str>,
}
impl RevokeChatInviteLink {
    /// Creates a new `RevokeChatInviteLink`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier of the target chat or username of the target channel in the format @username
    /// * `invite_link` - The invite link to revoke
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<Box<str>>>(
        chat_id: T0,
        invite_link: T1,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            invite_link: invite_link.into(),
        }
    }

    /// Unique identifier of the target chat or username of the target channel in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// The invite link to revoke
    #[must_use]
    pub fn invite_link<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.invite_link = val.into();
        this
    }
}
impl super::TelegramMethod for RevokeChatInviteLink {
    type Method = Self;
    type Return = crate::types::ChatInviteLink;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("revokeChatInviteLink", self, None)
    }
}
