/// Deletes a regular gift owned by a managed business account without receiving Telegram Stars for it. Requires the `can_convert_gifts_to_stars` business bot right. The gift can be deleted only if it isn't limited (the field `total_count` of the Gift is omitted) and wasn't refunded. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#deletebusinessaccountgift>
/// # Returns
/// - `bool`
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct DeleteBusinessAccountGift {
    /// Unique identifier of the business connection
    pub business_connection_id: Box<str>,
    /// Unique identifier of the regular gift that should be deleted
    pub owned_gift_id: Box<str>,
}
impl DeleteBusinessAccountGift {
    /// Creates a new `DeleteBusinessAccountGift`.
    ///
    /// # Arguments
    /// * `business_connection_id` - Unique identifier of the business connection
    /// * `owned_gift_id` - Unique identifier of the regular gift that should be deleted
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
    pub fn business_connection_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.business_connection_id = val.into();
        self
    }

    /// Unique identifier of the regular gift that should be deleted
    #[must_use]
    pub fn owned_gift_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.owned_gift_id = val.into();
        self
    }
}
impl super::TelegramMethod for DeleteBusinessAccountGift {
    type Method = Self;
    type Return = bool;

    fn build_request(self) -> super::Request<Self::Method> {
        super::Request::new("deleteBusinessAccountGift", self, None)
    }
}
