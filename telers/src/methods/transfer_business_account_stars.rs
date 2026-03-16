use crate::client::Bot;
use serde::Serialize;
/// Transfers Telegram Stars from the business account balance to the bot's balance. Requires the `can_transfer_stars` business bot right. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#transferbusinessaccountstars>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct TransferBusinessAccountStars {
    /// Unique identifier of the business connection
    pub business_connection_id: Box<str>,
    /// Number of Telegram Stars to transfer; 1-10000
    pub star_count: u16,
}
impl TransferBusinessAccountStars {
    /// Creates a new `TransferBusinessAccountStars`.
    ///
    /// # Arguments
    /// * `business_connection_id` - Unique identifier of the business connection
    /// * `star_count` - Number of Telegram Stars to transfer; 1-10000
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<u16>>(
        business_connection_id: T0,
        star_count: T1,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            star_count: star_count.into(),
        }
    }

    /// Unique identifier of the business connection
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.business_connection_id = val.into();
        this
    }

    /// Number of Telegram Stars to transfer; 1-10000
    #[must_use]
    pub fn star_count<T: Into<u16>>(self, val: T) -> Self {
        let mut this = self;
        this.star_count = val.into();
        this
    }
}
impl super::TelegramMethod for TransferBusinessAccountStars {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("transferBusinessAccountStars", self, None)
    }
}
