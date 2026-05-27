use crate::client::Bot;
use serde::Serialize;
/// Transfers an owned unique gift to another user. Requires the `can_transfer_and_upgrade_gifts` business bot right. Requires `can_transfer_stars` business bot right if the transfer is paid. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#transfergift>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct TransferGift {
    /// Unique identifier of the business connection
    pub business_connection_id: Box<str>,
    /// Unique identifier of the regular gift that should be transferred
    pub owned_gift_id: Box<str>,
    /// Unique identifier of the chat which will own the gift. The chat must be active in the last 24 hours.
    pub new_owner_chat_id: i64,
    /// The amount of Telegram Stars that will be paid for the transfer from the business account balance. If positive, then the `can_transfer_stars` business bot right is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_count: Option<i64>,
}
impl TransferGift {
    /// Creates a new `TransferGift`.
    ///
    /// # Arguments
    /// * `business_connection_id` - Unique identifier of the business connection
    /// * `owned_gift_id` - Unique identifier of the regular gift that should be transferred
    /// * `new_owner_chat_id` - Unique identifier of the chat which will own the gift. The chat must be active in the last 24 hours.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>, T2: Into<i64>>(
        business_connection_id: T0,
        owned_gift_id: T1,
        new_owner_chat_id: T2,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            owned_gift_id: owned_gift_id.into(),
            new_owner_chat_id: new_owner_chat_id.into(),
            star_count: None,
        }
    }

    /// Unique identifier of the business connection
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.business_connection_id = val.into();
        self
    }

    /// Unique identifier of the regular gift that should be transferred
    #[must_use]
    pub fn owned_gift_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.owned_gift_id = val.into();
        self
    }

    /// Unique identifier of the chat which will own the gift. The chat must be active in the last 24 hours.
    #[must_use]
    pub fn new_owner_chat_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.new_owner_chat_id = val.into();
        self
    }

    /// The amount of Telegram Stars that will be paid for the transfer from the business account balance. If positive, then the `can_transfer_stars` business bot right is required.
    #[must_use]
    pub fn star_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.star_count = Some(val.into());
        self
    }

    /// The amount of Telegram Stars that will be paid for the transfer from the business account balance. If positive, then the `can_transfer_stars` business bot right is required.
    #[must_use]
    pub fn star_count_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.star_count = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for TransferGift {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("transferGift", self, None)
    }
}
