use super::{InputProfilePhotoAnimated, InputProfilePhotoStatic};

use serde::Serialize;

/// This object describes a profile photo to set. Currently, it can be one of
/// - [`InputProfilePhotoStatic`]
/// - [`InputProfilePhotoAnimated`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputprofilephoto>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputProfilePhoto<'a> {
    Static(InputProfilePhotoStatic<'a>),
    Animated(InputProfilePhotoAnimated<'a>),
}

impl<'a> From<InputProfilePhotoStatic<'a>> for InputProfilePhoto<'a> {
    fn from(photo: InputProfilePhotoStatic<'a>) -> Self {
        Self::Static(photo)
    }
}

impl<'a> From<InputProfilePhotoAnimated<'a>> for InputProfilePhoto<'a> {
    fn from(photo: InputProfilePhotoAnimated<'a>) -> Self {
        Self::Animated(photo)
    }
}
