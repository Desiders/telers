use crate::types::InputPaidMedia;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object describes the paid media to be sent. Currently, it can be one of
/// - [`InputPaidMediaPhoto`]
/// - [`InputPaidMediaVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputpaidmedia>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum InputPaidMediaType {
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "video")]
    Video,
}
impl InputPaidMediaType {
    #[must_use]
    pub const fn all() -> [InputPaidMediaType; 2usize] {
        [InputPaidMediaType::Photo, InputPaidMediaType::Video]
    }
}
impl From<InputPaidMediaType> for Box<str> {
    fn from(val: InputPaidMediaType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InputPaidMediaType> for String {
    fn from(val: InputPaidMediaType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InputPaidMediaType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InputPaidMedia> for InputPaidMediaType {
    fn from(val: &'a InputPaidMedia) -> Self {
        match val {
            InputPaidMedia::Photo(_) => InputPaidMediaType::Photo,
            InputPaidMedia::Video(_) => InputPaidMediaType::Video,
        }
    }
}
impl From<InputPaidMedia> for InputPaidMediaType {
    fn from(val: InputPaidMedia) -> Self {
        InputPaidMediaType::from(&val)
    }
}
