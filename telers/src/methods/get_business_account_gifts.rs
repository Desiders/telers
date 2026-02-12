use super::base::{Request, TelegramMethod};
use crate::{client::Bot, types::OwnedGifts};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Returns the gifts received and owned by a managed business account. Requires the `can_view_gifts_and_stars` business bot right.
/// # Documentation
/// <https://core.telegram.org/bots/api#getbusinessaccountgifts>
/// # Returns
/// On success, [`OwnedGifts`] is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct GetBusinessAccountGifts {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Pass `true` to exclude gifts that aren't saved to the account's profile page
    pub exclude_unsaved: Option<bool>,
    /// Pass `true` to exclude gifts that are saved to the account's profile page
    pub exclude_saved: Option<bool>,
    /// Pass `true` to exclude gifts that can be purchased an unlimited number of times
    pub exclude_unlimited: Option<bool>,
    /// Pass `true` to exclude gifts that can be purchased a limited number of times
    pub exclude_limited: Option<bool>,
    /// Pass `true` to exclude unique gifts
    pub exclude_unique: Option<bool>,
    /// Pass `true` to sort results by gift price instead of send date. Sorting is applied before pagination.
    pub sort_by_price: Option<bool>,
    /// Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results
    pub offset: Option<Box<str>>,
    /// The maximum number of gifts to be returned; 1-100. Defaults to 100
    pub limit: Option<u8>,
}

impl GetBusinessAccountGifts {
    #[must_use]
    pub fn new(business_connection_id: impl Into<String>) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            exclude_unsaved: None,
            exclude_saved: None,
            exclude_unlimited: None,
            exclude_limited: None,
            exclude_unique: None,
            sort_by_price: None,
            offset: None,
            limit: None,
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
    pub fn exclude_unsaved(self, val: bool) -> Self {
        Self {
            exclude_unsaved: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn exclude_saved(self, val: bool) -> Self {
        Self {
            exclude_saved: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn exclude_unlimited(self, val: bool) -> Self {
        Self {
            exclude_unlimited: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn exclude_limited(self, val: bool) -> Self {
        Self {
            exclude_limited: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn exclude_unique(self, val: bool) -> Self {
        Self {
            exclude_unique: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn sort_by_price(self, val: bool) -> Self {
        Self {
            sort_by_price: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn offset(self, val: Box<str>) -> Self {
        Self {
            offset: Some(val),
            ..self
        }
    }
}

impl GetBusinessAccountGifts {
    #[must_use]
    pub fn exclude_unsaved_option(self, val: Option<bool>) -> Self {
        Self {
            exclude_unsaved: val,
            ..self
        }
    }

    #[must_use]
    pub fn exclude_saved_option(self, val: Option<bool>) -> Self {
        Self {
            exclude_saved: val,
            ..self
        }
    }

    #[must_use]
    pub fn exclude_unlimited_option(self, val: Option<bool>) -> Self {
        Self {
            exclude_unlimited: val,
            ..self
        }
    }

    #[must_use]
    pub fn exclude_limited_option(self, val: Option<bool>) -> Self {
        Self {
            exclude_limited: val,
            ..self
        }
    }

    #[must_use]
    pub fn exclude_unique_option(self, val: Option<bool>) -> Self {
        Self {
            exclude_unique: val,
            ..self
        }
    }

    #[must_use]
    pub fn sort_by_price_option(self, val: Option<bool>) -> Self {
        Self {
            sort_by_price: val,
            ..self
        }
    }

    #[must_use]
    pub fn offset_option(self, val: Option<Box<str>>) -> Self {
        Self {
            offset: val,
            ..self
        }
    }
}

impl TelegramMethod for GetBusinessAccountGifts {
    type Method = Self;
    type Return = OwnedGifts;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("getBusinessAccountGifts", self, None)
    }
}

impl AsRef<GetBusinessAccountGifts> for GetBusinessAccountGifts {
    fn as_ref(&self) -> &Self {
        self
    }
}
