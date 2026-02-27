use crate::client::Bot;
use serde::Serialize;
/// Changes the privacy settings pertaining to incoming gifts in a managed business account. Requires the `can_change_gift_settings` business bot right. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setbusinessaccountgiftsettings>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetBusinessAccountGiftSettings {
    /// Unique identifier of the business connection
    pub business_connection_id: Box<str>,
    /// Pass `true`, if a button for sending a gift to the user or by the business account must always be shown in the input field
    pub show_gift_button: bool,
    /// Types of gifts accepted by the business account
    pub accepted_gift_types: crate::types::AcceptedGiftTypes,
}
impl SetBusinessAccountGiftSettings {
    /// Creates a new `SetBusinessAccountGiftSettings`.
    ///
    /// # Arguments
    /// * `business_connection_id` - Unique identifier of the business connection
    /// * `show_gift_button` - Pass `true`, if a button for sending a gift to the user or by the business account must always be shown in the input field
    /// * `accepted_gift_types` - Types of gifts accepted by the business account
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<bool>, T2: Into<crate::types::AcceptedGiftTypes>>(
        business_connection_id: T0,
        show_gift_button: T1,
        accepted_gift_types: T2,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            show_gift_button: show_gift_button.into(),
            accepted_gift_types: accepted_gift_types.into(),
        }
    }

    /// Unique identifier of the business connection
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.business_connection_id = val.into();
        this
    }

    /// Pass `true`, if a button for sending a gift to the user or by the business account must always be shown in the input field
    #[must_use]
    pub fn show_gift_button<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.show_gift_button = val.into();
        this
    }

    /// Types of gifts accepted by the business account
    #[must_use]
    pub fn accepted_gift_types<T: Into<crate::types::AcceptedGiftTypes>>(self, val: T) -> Self {
        let mut this = self;
        this.accepted_gift_types = val.into();
        this
    }
}
impl super::TelegramMethod for SetBusinessAccountGiftSettings {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setBusinessAccountGiftSettings", self, None)
    }
}
