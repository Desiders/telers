use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Upgrades a given regular gift to a unique gift. Requires the `can_transfer_and_upgrade_gifts` business bot right. Additionally requires the `can_transfer_stars` business bot right if the upgrade is paid.
/// # Documentation
/// <https://core.telegram.org/bots/api#upgradegift>
/// # Returns
/// On success, `true` is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct UpgradeGift {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Unique identifier of the regular gift that should be upgraded to a unique one
    pub owned_gift_id: String,
    /// Pass `true` to keep the original gift text, sender and receiver in the upgraded gift
    pub keep_original_details: Option<bool>,
    /// The amount of Telegram Stars that will be paid for the upgrade from the business account balance. If `gift.prepaid_upgrade_star_count > 0`, then pass `0`, otherwise, the `can_transfer_stars` business bot right is required and `gift.upgrade_star_count` must be passed.
    pub star_count: Option<i64>,
}

impl UpgradeGift {
    #[must_use]
    pub fn new(
        business_connection_id: impl Into<String>,
        owned_gift_id: impl Into<String>,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            owned_gift_id: owned_gift_id.into(),
            keep_original_details: None,
            star_count: None,
        }
    }

    #[must_use]
    pub fn business_connection_id(self, val: impl Into<String>) -> Self {
        Self {
            business_connection_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn owned_gift_id(self, val: impl Into<String>) -> Self {
        Self {
            owned_gift_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn keep_original_details(self, val: bool) -> Self {
        Self {
            keep_original_details: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn star_count(self, val: i64) -> Self {
        Self {
            star_count: Some(val),
            ..self
        }
    }
}

impl UpgradeGift {
    pub fn keep_original_details_option(self, val: Option<bool>) -> Self {
        Self {
            keep_original_details: val,
            ..self
        }
    }

    pub fn star_count_option(self, val: Option<i64>) -> Self {
        Self {
            star_count: val,
            ..self
        }
    }
}

impl TelegramMethod for UpgradeGift {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("upgradeGift", self, None)
    }
}

impl AsRef<UpgradeGift> for UpgradeGift {
    fn as_ref(&self) -> &Self {
        self
    }
}
