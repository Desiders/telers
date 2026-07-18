use crate::types::PollMedia;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// At most one of the optional fields can be present in any given object.
/// Currently, it can be one of
/// - [`crate::types::PollMediaAnimation`]
/// - [`crate::types::PollMediaAudio`]
/// - [`crate::types::PollMediaDocument`]
/// - [`crate::types::PollMediaLink`]
/// - [`crate::types::PollMediaLivePhoto`]
/// - [`crate::types::PollMediaLocation`]
/// - [`crate::types::PollMediaPhoto`]
/// - [`crate::types::PollMediaSticker`]
/// - [`crate::types::PollMediaVenue`]
/// - [`crate::types::PollMediaVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#pollmedia>
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
pub enum PollMediaType {
    #[strum(serialize = "animation")]
    Animation,
    #[strum(serialize = "live_photo")]
    LivePhoto,
    #[strum(serialize = "venue")]
    Venue,
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "document")]
    Document,
    #[strum(serialize = "link")]
    Link,
    #[strum(serialize = "location")]
    Location,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "sticker")]
    Sticker,
    #[strum(serialize = "video")]
    Video,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl PollMediaType {
    #[must_use]
    pub const fn all() -> [PollMediaType; 11usize] {
        [
            PollMediaType::Animation,
            PollMediaType::LivePhoto,
            PollMediaType::Venue,
            PollMediaType::Audio,
            PollMediaType::Document,
            PollMediaType::Link,
            PollMediaType::Location,
            PollMediaType::Photo,
            PollMediaType::Sticker,
            PollMediaType::Video,
            PollMediaType::Unknown,
        ]
    }
}
impl From<PollMediaType> for Box<str> {
    fn from(val: PollMediaType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<PollMediaType> for String {
    fn from(val: PollMediaType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for PollMediaType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a PollMedia> for PollMediaType {
    fn from(val: &'a PollMedia) -> Self {
        match val {
            PollMedia::Animation(_) => PollMediaType::Animation,
            PollMedia::LivePhoto(_) => PollMediaType::LivePhoto,
            PollMedia::Venue(_) => PollMediaType::Venue,
            PollMedia::Audio(_) => PollMediaType::Audio,
            PollMedia::Document(_) => PollMediaType::Document,
            PollMedia::Link(_) => PollMediaType::Link,
            PollMedia::Location(_) => PollMediaType::Location,
            PollMedia::Photo(_) => PollMediaType::Photo,
            PollMedia::Sticker(_) => PollMediaType::Sticker,
            PollMedia::Video(_) => PollMediaType::Video,
            PollMedia::Unknown(_) => PollMediaType::Unknown,
        }
    }
}
impl From<PollMedia> for PollMediaType {
    fn from(val: PollMedia) -> Self {
        PollMediaType::from(&val)
    }
}
