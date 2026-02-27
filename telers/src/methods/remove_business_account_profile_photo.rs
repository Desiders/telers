use crate::client::Bot;
use serde::Serialize;
/// Removes the current profile photo of a managed business account. Requires the `can_edit_profile_photo` business bot right. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#removebusinessaccountprofilephoto>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct RemoveBusinessAccountProfilePhoto {
    /// Unique identifier of the business connection
    pub business_connection_id: Box<str>,
    /// Pass `true` to remove the public photo, which is visible even if the main photo is hidden by the business account's privacy settings. After the main photo is removed, the previous profile photo (if present) becomes the main photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
}
impl RemoveBusinessAccountProfilePhoto {
    /// Creates a new `RemoveBusinessAccountProfilePhoto`.
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
            is_public: None,
        }
    }

    /// Unique identifier of the business connection
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.business_connection_id = val.into();
        this
    }

    /// Pass `true` to remove the public photo, which is visible even if the main photo is hidden by the business account's privacy settings. After the main photo is removed, the previous profile photo (if present) becomes the main photo.
    #[must_use]
    pub fn is_public<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_public = Some(val.into());
        this
    }

    /// Pass `true` to remove the public photo, which is visible even if the main photo is hidden by the business account's privacy settings. After the main photo is removed, the previous profile photo (if present) becomes the main photo.
    #[must_use]
    pub fn is_public_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.is_public = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for RemoveBusinessAccountProfilePhoto {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("removeBusinessAccountProfilePhoto", self, None)
    }
}
