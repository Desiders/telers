use crate::types::InputRichMessageMediaContent;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object represents the media content of a rich message to be sent.
/// Currently, it can be one of
/// - [`crate::types::InputMediaAnimation`]
/// - [`crate::types::InputMediaAudio`]
/// - [`crate::types::InputMediaDocument`]
/// - [`crate::types::InputMediaPhoto`]
/// - [`crate::types::InputMediaVideo`]
/// - [`crate::types::InputMediaVoiceNote`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichmessagemedia>
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
pub enum InputRichMessageMediaContentType {
    #[strum(serialize = "animation")]
    Animation,
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "document")]
    Document,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "video")]
    Video,
    #[strum(serialize = "voice_note")]
    VoiceNote,
}
impl InputRichMessageMediaContentType {
    #[must_use]
    pub const fn all() -> [InputRichMessageMediaContentType; 6usize] {
        [
            InputRichMessageMediaContentType::Animation,
            InputRichMessageMediaContentType::Audio,
            InputRichMessageMediaContentType::Document,
            InputRichMessageMediaContentType::Photo,
            InputRichMessageMediaContentType::Video,
            InputRichMessageMediaContentType::VoiceNote,
        ]
    }
}
impl From<InputRichMessageMediaContentType> for Box<str> {
    fn from(val: InputRichMessageMediaContentType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InputRichMessageMediaContentType> for String {
    fn from(val: InputRichMessageMediaContentType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InputRichMessageMediaContentType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InputRichMessageMediaContent> for InputRichMessageMediaContentType {
    fn from(val: &'a InputRichMessageMediaContent) -> Self {
        match val {
            InputRichMessageMediaContent::Animation(_) => {
                InputRichMessageMediaContentType::Animation
            }
            InputRichMessageMediaContent::Audio(_) => InputRichMessageMediaContentType::Audio,
            InputRichMessageMediaContent::Document(_) => InputRichMessageMediaContentType::Document,
            InputRichMessageMediaContent::Photo(_) => InputRichMessageMediaContentType::Photo,
            InputRichMessageMediaContent::Video(_) => InputRichMessageMediaContentType::Video,
            InputRichMessageMediaContent::VoiceNote(_) => {
                InputRichMessageMediaContentType::VoiceNote
            }
        }
    }
}
impl From<InputRichMessageMediaContent> for InputRichMessageMediaContentType {
    fn from(val: InputRichMessageMediaContent) -> Self {
        InputRichMessageMediaContentType::from(&val)
    }
}
