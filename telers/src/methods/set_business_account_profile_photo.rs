use crate::client::Bot;
use serde::Serialize;
/// Changes the profile photo of a managed business account. Requires the `can_edit_profile_photo` business bot right. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setbusinessaccountprofilephoto>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetBusinessAccountProfilePhoto {
    /// Unique identifier of the business connection
    pub business_connection_id: Box<str>,
    /// The new profile photo to set
    pub photo: crate::types::InputProfilePhoto,
    /// Pass `true` to set the public photo, which will be visible even if the main photo is hidden by the business account's privacy settings. An account can have only one public photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
}
impl SetBusinessAccountProfilePhoto {
    /// Creates a new `SetBusinessAccountProfilePhoto`.
    ///
    /// # Arguments
    /// * `business_connection_id` - Unique identifier of the business connection
    /// * `photo` - The new profile photo to set
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<crate::types::InputProfilePhoto>>(
        business_connection_id: T0,
        photo: T1,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            photo: photo.into(),
            is_public: None,
        }
    }

    /// Unique identifier of the business connection
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.business_connection_id = val.into();
        self
    }

    /// The new profile photo to set
    #[must_use]
    pub fn photo<T: Into<crate::types::InputProfilePhoto>>(mut self, val: T) -> Self {
        self.photo = val.into();
        self
    }

    /// Pass `true` to set the public photo, which will be visible even if the main photo is hidden by the business account's privacy settings. An account can have only one public photo.
    #[must_use]
    pub fn is_public<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_public = Some(val.into());
        self
    }

    /// Pass `true` to set the public photo, which will be visible even if the main photo is hidden by the business account's privacy settings. An account can have only one public photo.
    #[must_use]
    pub fn is_public_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_public = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for SetBusinessAccountProfilePhoto {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setBusinessAccountProfilePhoto", self, None)
    }
}
