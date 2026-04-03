use crate::client::Bot;
use serde::Serialize;
/// Use this method to get the token of a managed bot. Returns the token as String on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#getmanagedbottoken>
/// # Returns
/// - `Box<str>`
#[derive(Clone, Debug, Serialize)]
pub struct GetManagedBotToken {
    /// User identifier of the managed bot whose token will be returned
    pub user_id: i64,
}
impl GetManagedBotToken {
    /// Creates a new `GetManagedBotToken`.
    ///
    /// # Arguments
    /// * `user_id` - User identifier of the managed bot whose token will be returned
    #[must_use]
    pub fn new<T0: Into<i64>>(user_id: T0) -> Self {
        Self {
            user_id: user_id.into(),
        }
    }

    /// User identifier of the managed bot whose token will be returned
    #[must_use]
    pub fn user_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.user_id = val.into();
        this
    }
}
impl super::TelegramMethod for GetManagedBotToken {
    type Method = Self;
    type Return = Box<str>;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getManagedBotToken", self, None)
    }
}
