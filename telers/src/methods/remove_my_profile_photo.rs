use crate::client::Bot;
use serde::Serialize;
/// Removes the profile photo of the bot. Requires no parameters. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#removemyprofilephoto>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct RemoveMyProfilePhoto {}
impl RemoveMyProfilePhoto {
    /// Creates a new `RemoveMyProfilePhoto`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for RemoveMyProfilePhoto {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for RemoveMyProfilePhoto {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("removeMyProfilePhoto", self, None)
    }
}
