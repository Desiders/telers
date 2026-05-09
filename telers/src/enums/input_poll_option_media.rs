use crate::types::InputPollOptionMedia;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object represents the content of a poll option to be sent. It should be one of
/// - [`crate::types::InputMediaAnimation`]
/// - [`crate::types::InputMediaLivePhoto`]
/// - [`crate::types::InputMediaLocation`]
/// - [`crate::types::InputMediaPhoto`]
/// - [`crate::types::InputMediaSticker`]
/// - [`crate::types::InputMediaVenue`]
/// - [`crate::types::InputMediaVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputpolloptionmedia>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum InputPollOptionMediaType {
    #[strum(serialize = "animation")]
    Animation,
    #[strum(serialize = "live_photo")]
    LivePhoto,
    #[strum(serialize = "location")]
    Location,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "sticker")]
    Sticker,
    #[strum(serialize = "venue")]
    Venue,
    #[strum(serialize = "video")]
    Video,
}
impl InputPollOptionMediaType {
    #[must_use]
    pub const fn all() -> [InputPollOptionMediaType; 7usize] {
        [
            InputPollOptionMediaType::Animation,
            InputPollOptionMediaType::LivePhoto,
            InputPollOptionMediaType::Location,
            InputPollOptionMediaType::Photo,
            InputPollOptionMediaType::Sticker,
            InputPollOptionMediaType::Venue,
            InputPollOptionMediaType::Video,
        ]
    }
}
impl From<InputPollOptionMediaType> for Box<str> {
    fn from(val: InputPollOptionMediaType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InputPollOptionMediaType> for String {
    fn from(val: InputPollOptionMediaType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InputPollOptionMediaType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InputPollOptionMedia> for InputPollOptionMediaType {
    fn from(val: &'a InputPollOptionMedia) -> Self {
        match val {
            InputPollOptionMedia::Animation(_) => InputPollOptionMediaType::Animation,
            InputPollOptionMedia::LivePhoto(_) => InputPollOptionMediaType::LivePhoto,
            InputPollOptionMedia::Location(_) => InputPollOptionMediaType::Location,
            InputPollOptionMedia::Photo(_) => InputPollOptionMediaType::Photo,
            InputPollOptionMedia::Sticker(_) => InputPollOptionMediaType::Sticker,
            InputPollOptionMedia::Venue(_) => InputPollOptionMediaType::Venue,
            InputPollOptionMedia::Video(_) => InputPollOptionMediaType::Video,
        }
    }
}
impl From<InputPollOptionMedia> for InputPollOptionMediaType {
    fn from(val: InputPollOptionMedia) -> Self {
        InputPollOptionMediaType::from(&val)
    }
}
