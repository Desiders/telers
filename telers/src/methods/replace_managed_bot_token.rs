use crate::client::Bot;
use serde::Serialize;
/// Use this method to revoke the current token of a managed bot and generate a new one. Returns the new token as String on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#replacemanagedbottoken>
/// # Returns
/// - `Box<str>`
#[derive(Clone, Debug, Serialize)]
pub struct ReplaceManagedBotToken {
    /// User identifier of the managed bot whose token will be replaced
    pub user_id: i64,
}
impl ReplaceManagedBotToken {
    /// Creates a new `ReplaceManagedBotToken`.
    ///
    /// # Arguments
    /// * `user_id` - User identifier of the managed bot whose token will be replaced
    #[must_use]
    pub fn new<T0: Into<i64>>(user_id: T0) -> Self {
        Self {
            user_id: user_id.into(),
        }
    }

    /// User identifier of the managed bot whose token will be replaced
    #[must_use]
    pub fn user_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.user_id = val.into();
        this
    }
}
impl super::TelegramMethod for ReplaceManagedBotToken {
    type Method = Self;
    type Return = Box<str>;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("replaceManagedBotToken", self, None)
    }
}
