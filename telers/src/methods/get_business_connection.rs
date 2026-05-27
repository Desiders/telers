use crate::client::Bot;
use serde::Serialize;
/// Use this method to get information about the connection of the bot with a business account. Returns a [`crate::types::BusinessConnection`] object on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#getbusinessconnection>
/// # Returns
/// - `crate::types::BusinessConnection`
#[derive(Clone, Debug, Serialize)]
pub struct GetBusinessConnection {
    /// Unique identifier of the business connection
    pub business_connection_id: Box<str>,
}
impl GetBusinessConnection {
    /// Creates a new `GetBusinessConnection`.
    ///
    /// # Arguments
    /// * `business_connection_id` - Unique identifier of the business connection
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(business_connection_id: T0) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
        }
    }

    /// Unique identifier of the business connection
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.business_connection_id = val.into();
        self
    }
}
impl super::TelegramMethod for GetBusinessConnection {
    type Method = Self;
    type Return = crate::types::BusinessConnection;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getBusinessConnection", self, None)
    }
}
