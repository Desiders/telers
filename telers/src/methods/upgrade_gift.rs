use crate::client::Bot;
use serde::Serialize;
/// Upgrades a given regular gift to a unique gift. Requires the `can_transfer_and_upgrade_gifts` business bot right. Additionally requires the `can_transfer_stars` business bot right if the upgrade is paid. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#upgradegift>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct UpgradeGift {
    /// Unique identifier of the business connection
    pub business_connection_id: Box<str>,
    /// Unique identifier of the regular gift that should be upgraded to a unique one
    pub owned_gift_id: Box<str>,
    /// Pass `true` to keep the original gift text, sender and receiver in the upgraded gift
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_original_details: Option<bool>,
    /// The amount of Telegram Stars that will be paid for the upgrade from the business account balance. If `gift.prepaid_upgrade_star_count` > 0, then pass 0, otherwise, the `can_transfer_stars` business bot right is required and `gift.upgrade_star_count` must be passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_count: Option<i64>,
}
impl UpgradeGift {
    /// Creates a new `UpgradeGift`.
    ///
    /// # Arguments
    /// * `business_connection_id` - Unique identifier of the business connection
    /// * `owned_gift_id` - Unique identifier of the regular gift that should be upgraded to a unique one
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(
        business_connection_id: T0,
        owned_gift_id: T1,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            owned_gift_id: owned_gift_id.into(),
            keep_original_details: None,
            star_count: None,
        }
    }

    /// Unique identifier of the business connection
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.business_connection_id = val.into();
        self
    }

    /// Unique identifier of the regular gift that should be upgraded to a unique one
    #[must_use]
    pub fn owned_gift_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.owned_gift_id = val.into();
        self
    }

    /// Pass `true` to keep the original gift text, sender and receiver in the upgraded gift
    #[must_use]
    pub fn keep_original_details<T: Into<bool>>(mut self, val: T) -> Self {
        self.keep_original_details = Some(val.into());
        self
    }

    /// Pass `true` to keep the original gift text, sender and receiver in the upgraded gift
    #[must_use]
    pub fn keep_original_details_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.keep_original_details = val.map(Into::into);
        self
    }

    /// The amount of Telegram Stars that will be paid for the upgrade from the business account balance. If `gift.prepaid_upgrade_star_count` > 0, then pass 0, otherwise, the `can_transfer_stars` business bot right is required and `gift.upgrade_star_count` must be passed.
    #[must_use]
    pub fn star_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.star_count = Some(val.into());
        self
    }

    /// The amount of Telegram Stars that will be paid for the upgrade from the business account balance. If `gift.prepaid_upgrade_star_count` > 0, then pass 0, otherwise, the `can_transfer_stars` business bot right is required and `gift.upgrade_star_count` must be passed.
    #[must_use]
    pub fn star_count_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.star_count = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for UpgradeGift {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("upgradeGift", self, None)
    }
}
