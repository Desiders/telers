/// A simple method for testing your bot's authentication token. Requires no parameters. Returns basic information about the bot in form of a User object.
/// # Documentation
/// <https://core.telegram.org/bots/api#getme>
/// # Returns
/// - `crate::types::User`
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct GetMe {}
impl GetMe {
    /// Creates a new `GetMe`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for GetMe {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for GetMe {
    type Method = Self;
    type Return = crate::types::User;

    fn build_request(self) -> super::Request<Self::Method> {
        super::Request::new("getMe", self, None)
    }
}
