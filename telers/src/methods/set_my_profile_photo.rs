use crate::client::Bot;
use serde::Serialize;
/// Changes the profile photo of the bot. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setmyprofilephoto>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetMyProfilePhoto {
    /// The new profile photo to set
    pub photo: crate::types::InputProfilePhoto,
}
impl SetMyProfilePhoto {
    /// Creates a new `SetMyProfilePhoto`.
    ///
    /// # Arguments
    /// * `photo` - The new profile photo to set
    #[must_use]
    pub fn new<T0: Into<crate::types::InputProfilePhoto>>(photo: T0) -> Self {
        Self {
            photo: photo.into(),
        }
    }

    /// The new profile photo to set
    #[must_use]
    pub fn photo<T: Into<crate::types::InputProfilePhoto>>(self, val: T) -> Self {
        let mut this = self;
        this.photo = val.into();
        this
    }
}
impl super::TelegramMethod for SetMyProfilePhoto {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setMyProfilePhoto", self, None)
    }
}
