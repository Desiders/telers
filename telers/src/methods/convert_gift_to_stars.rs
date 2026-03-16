use crate::client::Bot;
use serde::Serialize;
/// Converts a given regular gift to Telegram Stars. Requires the `can_convert_gifts_to_stars` business bot right. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#convertgifttostars>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct ConvertGiftToStars {
    /// Unique identifier of the business connection
    pub business_connection_id: Box<str>,
    /// Unique identifier of the regular gift that should be converted to Telegram Stars
    pub owned_gift_id: Box<str>,
}
impl ConvertGiftToStars {
    /// Creates a new `ConvertGiftToStars`.
    ///
    /// # Arguments
    /// * `business_connection_id` - Unique identifier of the business connection
    /// * `owned_gift_id` - Unique identifier of the regular gift that should be converted to Telegram Stars
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(
        business_connection_id: T0,
        owned_gift_id: T1,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            owned_gift_id: owned_gift_id.into(),
        }
    }

    /// Unique identifier of the business connection
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.business_connection_id = val.into();
        this
    }

    /// Unique identifier of the regular gift that should be converted to Telegram Stars
    #[must_use]
    pub fn owned_gift_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.owned_gift_id = val.into();
        this
    }
}
impl super::TelegramMethod for ConvertGiftToStars {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("convertGiftToStars", self, None)
    }
}
