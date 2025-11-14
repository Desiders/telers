use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::InputProfilePhoto};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Changes the profile photo of a managed business account. Requires the `can_edit_profile_photo` business bot right.
/// # Documentation
/// <https://core.telegram.org/bots/api#setbusinessaccountprofilephoto>
/// # Returns
/// On success, `true` is returned
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SetBusinessAccountProfilePhoto<'a> {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// The new profile photo to set
    pub photo: InputProfilePhoto<'a>,
    /// Pass `true` to set the public photo, which will be visible even if the main photo is hidden by the business account's privacy settings. An account can have only one public photo.
    pub is_public: Option<bool>,
}

impl<'a> SetBusinessAccountProfilePhoto<'a> {
    #[must_use]
    pub fn new(
        business_connection_id: impl Into<String>,
        photo: impl Into<InputProfilePhoto<'a>>,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            photo: photo.into(),
            is_public: None,
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
    pub fn photo(self, val: impl Into<InputProfilePhoto<'a>>) -> Self {
        Self {
            photo: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn is_public(self, val: bool) -> Self {
        Self {
            is_public: Some(val),
            ..self
        }
    }
}

impl SetBusinessAccountProfilePhoto<'_> {
    #[must_use]
    pub fn is_public_option(self, val: Option<bool>) -> Self {
        Self {
            is_public: val,
            ..self
        }
    }
}

impl TelegramMethod for SetBusinessAccountProfilePhoto<'_> {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<'_, Self::Method> {
        Request::new("setBusinessAccountProfilePhoto", self, None)
    }
}

impl<'a> AsRef<SetBusinessAccountProfilePhoto<'a>> for SetBusinessAccountProfilePhoto<'a> {
    fn as_ref(&self) -> &Self {
        self
    }
}
