use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Transfers an owned unique gift to another user. Requires the `can_transfer_and_upgrade_gifts` business bot right. Requires `can_transfer_stars` business bot right if the transfer is paid.
/// # Documentation
/// <https://core.telegram.org/bots/api#transfergift>
/// # Returns
/// On success, `true` is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct TransferGift {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Unique identifier of the regular gift that should be transferred
    pub owned_gift_id: String,
    /// Unique identifier of the chat which will own the gift. The chat must be active in the last 24 hours.
    pub new_owner_chat_id: Option<i64>,
    /// The amount of Telegram Stars that will be paid for the transfer from the business account balance. If positive, then the `can_transfer_stars` business bot right is required.
    pub star_count: Option<i64>,
}

impl TransferGift {
    #[must_use]
    pub fn new(
        business_connection_id: impl Into<String>,
        owned_gift_id: impl Into<String>,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            owned_gift_id: owned_gift_id.into(),
            new_owner_chat_id: None,
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
    pub fn new_owner_chat_id(self, val: i64) -> Self {
        Self {
            new_owner_chat_id: Some(val),
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

impl TransferGift {
    #[must_use]
    pub fn new_owner_chat_id_option(self, val: Option<i64>) -> Self {
        Self {
            new_owner_chat_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn star_count_option(self, val: Option<i64>) -> Self {
        Self {
            star_count: val,
            ..self
        }
    }
}

impl TelegramMethod for TransferGift {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("transferGift", self, None)
    }
}

impl AsRef<TransferGift> for TransferGift {
    fn as_ref(&self) -> &Self {
        self
    }
}
