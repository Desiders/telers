use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;

/// Converts a given regular gift to Telegram Stars. Requires the `can_convert_gifts_to_stars` business bot right.
/// # Documentation
/// <https://core.telegram.org/bots/api#convertgifttostars>
/// # Returns
/// On success, `true` is returned
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct ConvertGiftToStars {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Unique identifier of the regular gift that should be converted to Telegram Stars
    pub owned_gift_id: String,
}

impl ConvertGiftToStars {
    #[must_use]
    pub fn new(
        business_connection_id: impl Into<String>,
        owned_gift_id: impl Into<String>,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            owned_gift_id: owned_gift_id.into(),
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
}

impl TelegramMethod for ConvertGiftToStars {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("convertGiftToStars", self, None)
    }
}

impl AsRef<ConvertGiftToStars> for ConvertGiftToStars {
    fn as_ref(&self) -> &Self {
        self
    }
}
