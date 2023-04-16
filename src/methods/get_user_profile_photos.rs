use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::UserProfilePhotos};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to get a list of profile pictures for a user.
/// # Documentation
/// <https://core.telegram.org/bots/api#getuserprofilephotos>
/// # Returns
/// Returns a [`UserProfilePhotos`] object
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct GetUserProfilePhotos {
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Sequential number of the first photo to be returned. By default, all photos are returned.
    pub offset: Option<i64>,
    /// Limits the number of photos to be retrieved. Values between 1—100 are accepted. Defaults to 100.
    pub limit: Option<i64>,
}

impl GetUserProfilePhotos {
    #[must_use]
    pub fn new(user_id: i64) -> Self {
        Self {
            user_id,
            offset: None,
            limit: None,
        }
    }

    #[must_use]
    pub fn user_id(self, val: i64) -> Self {
        Self {
            user_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn offset(self, val: i64) -> Self {
        Self {
            offset: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn limit(self, val: i64) -> Self {
        Self {
            limit: Some(val),
            ..self
        }
    }
}

impl GetUserProfilePhotos {
    #[must_use]
    pub fn offset_option(self, val: Option<i64>) -> Self {
        Self {
            offset: val,
            ..self
        }
    }

    #[must_use]
    pub fn limit_option(self, val: Option<i64>) -> Self {
        Self { limit: val, ..self }
    }
}

impl TelegramMethod for GetUserProfilePhotos {
    type Method = Self;
    type Return = UserProfilePhotos;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("getUserProfilePhotos", self, None)
    }
}
