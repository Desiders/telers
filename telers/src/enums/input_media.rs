use crate::types::InputMedia;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object represents the content of a media message to be sent. It should be one of
/// - [`crate::types::InputMediaAnimation`]
/// - [`crate::types::InputMediaDocument`]
/// - [`crate::types::InputMediaAudio`]
/// - [`crate::types::InputMediaPhoto`]
/// - [`crate::types::InputMediaVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputmedia>
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
pub enum InputMediaType {
    #[strum(serialize = "animation")]
    Animation,
    #[strum(serialize = "document")]
    Document,
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "video")]
    Video,
}
impl InputMediaType {
    #[must_use]
    pub const fn all() -> [InputMediaType; 5usize] {
        [
            InputMediaType::Animation,
            InputMediaType::Document,
            InputMediaType::Audio,
            InputMediaType::Photo,
            InputMediaType::Video,
        ]
    }
}
impl From<InputMediaType> for Box<str> {
    fn from(val: InputMediaType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InputMediaType> for String {
    fn from(val: InputMediaType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InputMediaType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InputMedia> for InputMediaType {
    fn from(val: &'a InputMedia) -> Self {
        match val {
            InputMedia::Animation(_) => InputMediaType::Animation,
            InputMedia::Document(_) => InputMediaType::Document,
            InputMedia::Audio(_) => InputMediaType::Audio,
            InputMedia::Photo(_) => InputMediaType::Photo,
            InputMedia::Video(_) => InputMediaType::Video,
        }
    }
}
impl From<InputMedia> for InputMediaType {
    fn from(val: InputMedia) -> Self {
        InputMediaType::from(&val)
    }
}
