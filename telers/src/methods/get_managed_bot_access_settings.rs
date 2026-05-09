use crate::client::Bot;
use serde::Serialize;
/// Use this method to get the access settings of a managed bot. Returns a [`crate::types::BotAccessSettings`] object on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#getmanagedbotaccesssettings>
/// # Returns
/// - `crate::types::BotAccessSettings`
#[derive(Clone, Debug, Serialize)]
pub struct GetManagedBotAccessSettings {
    /// User identifier of the managed bot whose access settings will be returned
    pub user_id: i64,
}
impl GetManagedBotAccessSettings {
    /// Creates a new `GetManagedBotAccessSettings`.
    ///
    /// # Arguments
    /// * `user_id` - User identifier of the managed bot whose access settings will be returned
    #[must_use]
    pub fn new<T0: Into<i64>>(user_id: T0) -> Self {
        Self {
            user_id: user_id.into(),
        }
    }

    /// User identifier of the managed bot whose access settings will be returned
    #[must_use]
    pub fn user_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.user_id = val.into();
        this
    }
}
impl super::TelegramMethod for GetManagedBotAccessSettings {
    type Method = Self;
    type Return = crate::types::BotAccessSettings;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getManagedBotAccessSettings", self, None)
    }
}
