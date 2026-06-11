use crate::types::InputPollMedia;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object represents the content of a poll description or a quiz explanation to be sent. It should be one of
/// - [`crate::types::InputMediaAnimation`]
/// - [`crate::types::InputMediaAudio`]
/// - [`crate::types::InputMediaDocument`]
/// - [`crate::types::InputMediaLivePhoto`]
/// - [`crate::types::InputMediaLocation`]
/// - [`crate::types::InputMediaPhoto`]
/// - [`crate::types::InputMediaVenue`]
/// - [`crate::types::InputMediaVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputpollmedia>
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
pub enum InputPollMediaType {
    #[strum(serialize = "animation")]
    Animation,
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "document")]
    Document,
    #[strum(serialize = "live_photo")]
    LivePhoto,
    #[strum(serialize = "location")]
    Location,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "venue")]
    Venue,
    #[strum(serialize = "video")]
    Video,
}
impl InputPollMediaType {
    #[must_use]
    pub const fn all() -> [InputPollMediaType; 8usize] {
        [
            InputPollMediaType::Animation,
            InputPollMediaType::Audio,
            InputPollMediaType::Document,
            InputPollMediaType::LivePhoto,
            InputPollMediaType::Location,
            InputPollMediaType::Photo,
            InputPollMediaType::Venue,
            InputPollMediaType::Video,
        ]
    }
}
impl From<InputPollMediaType> for Box<str> {
    fn from(val: InputPollMediaType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InputPollMediaType> for String {
    fn from(val: InputPollMediaType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InputPollMediaType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InputPollMedia> for InputPollMediaType {
    fn from(val: &'a InputPollMedia) -> Self {
        match val {
            InputPollMedia::Animation(_) => InputPollMediaType::Animation,
            InputPollMedia::Audio(_) => InputPollMediaType::Audio,
            InputPollMedia::Document(_) => InputPollMediaType::Document,
            InputPollMedia::LivePhoto(_) => InputPollMediaType::LivePhoto,
            InputPollMedia::Location(_) => InputPollMediaType::Location,
            InputPollMedia::Photo(_) => InputPollMediaType::Photo,
            InputPollMedia::Venue(_) => InputPollMediaType::Venue,
            InputPollMedia::Video(_) => InputPollMediaType::Video,
        }
    }
}
impl From<InputPollMedia> for InputPollMediaType {
    fn from(val: InputPollMedia) -> Self {
        InputPollMediaType::from(&val)
    }
}
