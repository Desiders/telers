use crate::types::InputStoryContent;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object describes the content of a story to post. Currently, it can be one of
/// - [`crate::types::InputStoryContentPhoto`]
/// - [`crate::types::InputStoryContentVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputstorycontent>
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
pub enum InputStoryContentType {
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "video")]
    Video,
}
impl InputStoryContentType {
    #[must_use]
    pub const fn all() -> [InputStoryContentType; 2usize] {
        [InputStoryContentType::Photo, InputStoryContentType::Video]
    }
}
impl From<InputStoryContentType> for Box<str> {
    fn from(val: InputStoryContentType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InputStoryContentType> for String {
    fn from(val: InputStoryContentType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InputStoryContentType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InputStoryContent> for InputStoryContentType {
    fn from(val: &'a InputStoryContent) -> Self {
        match val {
            InputStoryContent::Photo(_) => InputStoryContentType::Photo,
            InputStoryContent::Video(_) => InputStoryContentType::Video,
        }
    }
}
impl From<InputStoryContent> for InputStoryContentType {
    fn from(val: InputStoryContent) -> Self {
        InputStoryContentType::from(&val)
    }
}
