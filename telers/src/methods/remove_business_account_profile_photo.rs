use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Removes the current profile photo of a managed business account. Requires the `can_edit_profile_photo` business bot right.
/// # Documentation
/// <https://core.telegram.org/bots/api#removebusinessaccountprofilephoto>
/// # Returns
/// On success, `true` is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct RemoveBusinessAccountProfilePhoto {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Pass `true` to remove the public photo, which is visible even if the main photo is hidden by the business account's privacy settings. After the main photo is removed, the previous profile photo (if present) becomes the main photo.
    pub is_public: Option<bool>,
}

impl RemoveBusinessAccountProfilePhoto {
    #[must_use]
    pub fn new(business_connection_id: impl Into<String>) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
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
    pub fn is_public(self, val: bool) -> Self {
        Self {
            is_public: Some(val),
            ..self
        }
    }
}

impl RemoveBusinessAccountProfilePhoto {
    #[must_use]
    pub fn is_public_option(self, val: Option<bool>) -> Self {
        Self {
            is_public: val,
            ..self
        }
    }
}

impl TelegramMethod for RemoveBusinessAccountProfilePhoto {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("removeBusinessAccountProfilePhoto", self, None)
    }
}

impl AsRef<RemoveBusinessAccountProfilePhoto> for RemoveBusinessAccountProfilePhoto {
    fn as_ref(&self) -> &Self {
        self
    }
}
