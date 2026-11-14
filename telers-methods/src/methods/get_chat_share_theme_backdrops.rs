/// Use this method to get the list of backdrops, which can be used as the background of a theme of the screen for sharing a link to a user or a chat. Requires no parameters. Returns a [`crate::types::ChatShareThemeBackdrops`] object.
/// # Documentation
/// <https://core.telegram.org/bots/api#getchatsharethemebackdrops>
/// # Returns
/// - `crate::types::ChatShareThemeBackdrops`
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct GetChatShareThemeBackdrops {}
impl GetChatShareThemeBackdrops {
    /// Creates a new `GetChatShareThemeBackdrops`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for GetChatShareThemeBackdrops {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for GetChatShareThemeBackdrops {
    type Method = Self;
    type Return = crate::types::ChatShareThemeBackdrops;

    fn build_request(self) -> super::Request<Self::Method> {
        super::Request::new("getChatShareThemeBackdrops", self, None)
    }
}
