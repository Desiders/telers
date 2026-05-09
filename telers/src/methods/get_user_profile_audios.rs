use crate::client::Bot;
use serde::Serialize;
/// Use this method to get a list of profile audios for a user. Returns a [`crate::types::UserProfileAudios`] object.
/// # Documentation
/// <https://core.telegram.org/bots/api#getuserprofileaudios>
/// # Returns
/// - `crate::types::UserProfileAudios`
#[derive(Clone, Debug, Serialize)]
pub struct GetUserProfileAudios {
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Sequential number of the first audio to be returned. By default, all audios are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Limits the number of audios to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
}
impl GetUserProfileAudios {
    /// Creates a new `GetUserProfileAudios`.
    ///
    /// # Arguments
    /// * `user_id` - Unique identifier of the target user
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>>(user_id: T0) -> Self {
        Self {
            user_id: user_id.into(),
            offset: None,
            limit: None,
        }
    }

    /// Unique identifier of the target user
    #[must_use]
    pub fn user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.user_id = val.into();
        self
    }

    /// Sequential number of the first audio to be returned. By default, all audios are returned.
    #[must_use]
    pub fn offset<T: Into<i64>>(mut self, val: T) -> Self {
        self.offset = Some(val.into());
        self
    }

    /// Sequential number of the first audio to be returned. By default, all audios are returned.
    #[must_use]
    pub fn offset_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.offset = val.map(Into::into);
        self
    }

    /// Limits the number of audios to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[must_use]
    pub fn limit<T: Into<u8>>(mut self, val: T) -> Self {
        self.limit = Some(val.into());
        self
    }

    /// Limits the number of audios to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[must_use]
    pub fn limit_option<T: Into<u8>>(mut self, val: Option<T>) -> Self {
        self.limit = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for GetUserProfileAudios {
    type Method = Self;
    type Return = crate::types::UserProfileAudios;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getUserProfileAudios", self, None)
    }
}
