use crate::types::InputProfilePhoto;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object describes a profile photo to set. Currently, it can be one of
/// - [`crate::types::InputProfilePhotoStatic`]
/// - [`crate::types::InputProfilePhotoAnimated`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputprofilephoto>
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    AsRefStr,
    IntoStaticStr,
    Deserialize,
    Serialize,
)]
pub enum InputProfilePhotoType {
    #[strum(serialize = "static")]
    Static,
    #[strum(serialize = "animated")]
    Animated,
}
impl InputProfilePhotoType {
    #[must_use]
    pub const fn all() -> [InputProfilePhotoType; 2usize] {
        [
            InputProfilePhotoType::Static,
            InputProfilePhotoType::Animated,
        ]
    }
}
impl From<InputProfilePhotoType> for Box<str> {
    fn from(val: InputProfilePhotoType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InputProfilePhotoType> for String {
    fn from(val: InputProfilePhotoType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InputProfilePhotoType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InputProfilePhoto> for InputProfilePhotoType {
    fn from(val: &'a InputProfilePhoto) -> Self {
        match val {
            InputProfilePhoto::Static(_) => InputProfilePhotoType::Static,
            InputProfilePhoto::Animated(_) => InputProfilePhotoType::Animated,
        }
    }
}
impl From<InputProfilePhoto> for InputProfilePhotoType {
    fn from(val: InputProfilePhoto) -> Self {
        InputProfilePhotoType::from(&val)
    }
}
