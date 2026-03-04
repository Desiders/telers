use crate::client::Bot;
use serde::Serialize;
/// Returns the gifts received and owned by a managed business account. Requires the `can_view_gifts_and_stars` business bot right. Returns [`crate::types::OwnedGifts`] on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#getbusinessaccountgifts>
/// # Returns
/// - `crate::types::OwnedGifts`
#[derive(Clone, Debug, Serialize)]
pub struct GetBusinessAccountGifts {
    /// Unique identifier of the business connection
    pub business_connection_id: Box<str>,
    /// Pass `true` to exclude gifts that aren't saved to the account's profile page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_unsaved: Option<bool>,
    /// Pass `true` to exclude gifts that are saved to the account's profile page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_saved: Option<bool>,
    /// Pass `true` to exclude gifts that can be purchased an unlimited number of times
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_unlimited: Option<bool>,
    /// Pass `true` to exclude gifts that can be purchased a limited number of times and can be upgraded to unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_limited_upgradable: Option<bool>,
    /// Pass `true` to exclude gifts that can be purchased a limited number of times and can't be upgraded to unique
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_limited_non_upgradable: Option<bool>,
    /// Pass `true` to exclude unique gifts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_unique: Option<bool>,
    /// Pass `true` to exclude gifts that were assigned from the TON blockchain and can't be resold or transferred in Telegram
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_from_blockchain: Option<bool>,
    /// Pass `true` to sort results by gift price instead of send date. Sorting is applied before pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by_price: Option<bool>,
    /// Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Box<str>>,
    /// The maximum number of gifts to be returned; 1-100. Defaults to 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
}
impl GetBusinessAccountGifts {
    /// Creates a new `GetBusinessAccountGifts`.
    ///
    /// # Arguments
    /// * `business_connection_id` - Unique identifier of the business connection
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(business_connection_id: T0) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            exclude_unsaved: None,
            exclude_saved: None,
            exclude_unlimited: None,
            exclude_limited_upgradable: None,
            exclude_limited_non_upgradable: None,
            exclude_unique: None,
            exclude_from_blockchain: None,
            sort_by_price: None,
            offset: None,
            limit: None,
        }
    }

    /// Unique identifier of the business connection
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.business_connection_id = val.into();
        this
    }

    /// Pass `true` to exclude gifts that aren't saved to the account's profile page
    #[must_use]
    pub fn exclude_unsaved<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.exclude_unsaved = Some(val.into());
        this
    }

    /// Pass `true` to exclude gifts that aren't saved to the account's profile page
    #[must_use]
    pub fn exclude_unsaved_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.exclude_unsaved = val.map(Into::into);
        this
    }

    /// Pass `true` to exclude gifts that are saved to the account's profile page
    #[must_use]
    pub fn exclude_saved<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.exclude_saved = Some(val.into());
        this
    }

    /// Pass `true` to exclude gifts that are saved to the account's profile page
    #[must_use]
    pub fn exclude_saved_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.exclude_saved = val.map(Into::into);
        this
    }

    /// Pass `true` to exclude gifts that can be purchased an unlimited number of times
    #[must_use]
    pub fn exclude_unlimited<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.exclude_unlimited = Some(val.into());
        this
    }

    /// Pass `true` to exclude gifts that can be purchased an unlimited number of times
    #[must_use]
    pub fn exclude_unlimited_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.exclude_unlimited = val.map(Into::into);
        this
    }

    /// Pass `true` to exclude gifts that can be purchased a limited number of times and can be upgraded to unique
    #[must_use]
    pub fn exclude_limited_upgradable<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.exclude_limited_upgradable = Some(val.into());
        this
    }

    /// Pass `true` to exclude gifts that can be purchased a limited number of times and can be upgraded to unique
    #[must_use]
    pub fn exclude_limited_upgradable_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.exclude_limited_upgradable = val.map(Into::into);
        this
    }

    /// Pass `true` to exclude gifts that can be purchased a limited number of times and can't be upgraded to unique
    #[must_use]
    pub fn exclude_limited_non_upgradable<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.exclude_limited_non_upgradable = Some(val.into());
        this
    }

    /// Pass `true` to exclude gifts that can be purchased a limited number of times and can't be upgraded to unique
    #[must_use]
    pub fn exclude_limited_non_upgradable_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.exclude_limited_non_upgradable = val.map(Into::into);
        this
    }

    /// Pass `true` to exclude unique gifts
    #[must_use]
    pub fn exclude_unique<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.exclude_unique = Some(val.into());
        this
    }

    /// Pass `true` to exclude unique gifts
    #[must_use]
    pub fn exclude_unique_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.exclude_unique = val.map(Into::into);
        this
    }

    /// Pass `true` to exclude gifts that were assigned from the TON blockchain and can't be resold or transferred in Telegram
    #[must_use]
    pub fn exclude_from_blockchain<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.exclude_from_blockchain = Some(val.into());
        this
    }

    /// Pass `true` to exclude gifts that were assigned from the TON blockchain and can't be resold or transferred in Telegram
    #[must_use]
    pub fn exclude_from_blockchain_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.exclude_from_blockchain = val.map(Into::into);
        this
    }

    /// Pass `true` to sort results by gift price instead of send date. Sorting is applied before pagination.
    #[must_use]
    pub fn sort_by_price<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.sort_by_price = Some(val.into());
        this
    }

    /// Pass `true` to sort results by gift price instead of send date. Sorting is applied before pagination.
    #[must_use]
    pub fn sort_by_price_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.sort_by_price = val.map(Into::into);
        this
    }

    /// Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
    #[must_use]
    pub fn offset<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.offset = Some(val.into());
        this
    }

    /// Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
    #[must_use]
    pub fn offset_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.offset = val.map(Into::into);
        this
    }

    /// The maximum number of gifts to be returned; 1-100. Defaults to 100
    #[must_use]
    pub fn limit<T: Into<u8>>(self, val: T) -> Self {
        let mut this = self;
        this.limit = Some(val.into());
        this
    }

    /// The maximum number of gifts to be returned; 1-100. Defaults to 100
    #[must_use]
    pub fn limit_option<T: Into<u8>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.limit = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for GetBusinessAccountGifts {
    type Method = Self;
    type Return = crate::types::OwnedGifts;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getBusinessAccountGifts", self, None)
    }
}
