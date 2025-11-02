use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::AcceptedGiftTypes};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Changes the privacy settings pertaining to incoming gifts in a managed business account. Requires the `can_change_gift_settings` business bot right.
/// # Documentation
/// <https://core.telegram.org/bots/api#setbusinessaccountgiftsettings>
/// # Returns
/// On success, `true` is returned
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SetBusinessAccountGiftSettings {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Pass `true`, if a button for sending a gift to the user or by the business account must always be shown in the input field
    pub show_gift_button: bool,
    /// Types of gifts accepted by the business account
    pub accepted_gift_types: AcceptedGiftTypes,
}

impl SetBusinessAccountGiftSettings {
    #[must_use]
    pub fn new(
        business_connection_id: impl Into<String>,
        show_gift_button: bool,
        accepted_gift_types: AcceptedGiftTypes,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            show_gift_button,
            accepted_gift_types,
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
    pub fn show_gift_button(self, val: bool) -> Self {
        Self {
            show_gift_button: val,
            ..self
        }
    }

    #[must_use]
    pub fn accepted_gift_types(self, val: AcceptedGiftTypes) -> Self {
        Self {
            accepted_gift_types: val,
            ..self
        }
    }
}

impl TelegramMethod for SetBusinessAccountGiftSettings {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setBusinessAccountGiftSettings", self, None)
    }
}

impl AsRef<SetBusinessAccountGiftSettings> for SetBusinessAccountGiftSettings {
    fn as_ref(&self) -> &Self {
        self
    }
}
