use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// This enum represents all possible types of the input media
/// # Documentation
/// <https://core.telegram.org/bots/api#inputmedia>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum InputMediaType {
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
}

impl InputMediaType {
    #[must_use]
    pub const fn all() -> [InputMediaType; 5] {
        [
            InputMediaType::Animation,
            InputMediaType::Audio,
            InputMediaType::Document,
            InputMediaType::Photo,
            InputMediaType::Video,
        ]
    }
}

impl From<InputMediaType> for Box<str> {
    fn from(media_type: InputMediaType) -> Self {
        Into::<&'static str>::into(media_type).into()
    }
}

impl<'a> PartialEq<&'a str> for InputMediaType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
