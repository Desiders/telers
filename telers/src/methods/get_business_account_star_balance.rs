use crate::client::Bot;
use serde::Serialize;
/// Returns the amount of Telegram Stars owned by a managed business account. Requires the `can_view_gifts_and_stars` business bot right. Returns [`crate::types::StarAmount`] on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#getbusinessaccountstarbalance>
/// # Returns
/// - `crate::types::StarAmount`
#[derive(Clone, Debug, Serialize)]
pub struct GetBusinessAccountStarBalance {
    /// Unique identifier of the business connection
    pub business_connection_id: Box<str>,
}
impl GetBusinessAccountStarBalance {
    /// Creates a new `GetBusinessAccountStarBalance`.
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
impl super::TelegramMethod for GetBusinessAccountStarBalance {
    type Method = Self;
    type Return = crate::types::StarAmount;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getBusinessAccountStarBalance", self, None)
    }
}
