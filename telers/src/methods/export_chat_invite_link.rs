use crate::client::Bot;
use serde::Serialize;
/// Use this method to generate a new primary invite link for a chat; any previously generated primary link is revoked. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the new invite link as String on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#exportchatinvitelink>
/// # Returns
/// - `Box<str>`
#[derive(Clone, Debug, Serialize)]
pub struct ExportChatInviteLink {
    /// Unique identifier for the target chat or username of the target channel in the format @username
    pub chat_id: crate::types::ChatIdKind,
}
impl ExportChatInviteLink {
    /// Creates a new `ExportChatInviteLink`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel in the format @username
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>>(chat_id: T0) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }

    /// Unique identifier for the target chat or username of the target channel in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }
}
impl super::TelegramMethod for ExportChatInviteLink {
    type Method = Self;
    type Return = Box<str>;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("exportChatInviteLink", self, None)
    }
}
