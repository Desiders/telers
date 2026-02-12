use super::{InputProfilePhotoAnimated, InputProfilePhotoStatic};

use serde::Serialize;

/// This object describes a profile photo to set. Currently, it can be one of
/// - [`InputProfilePhotoStatic`]
/// - [`InputProfilePhotoAnimated`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputprofilephoto>
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputProfilePhoto {
    Static(InputProfilePhotoStatic),
    Animated(InputProfilePhotoAnimated),
}

impl From<InputProfilePhotoStatic> for InputProfilePhoto {
    fn from(photo: InputProfilePhotoStatic) -> Self {
        Self::Static(photo)
    }
}

impl From<InputProfilePhotoAnimated> for InputProfilePhoto {
    fn from(photo: InputProfilePhotoAnimated) -> Self {
        Self::Animated(photo)
    }
}
