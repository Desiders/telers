use std::{
    fmt::{self, Debug, Display},
    ops::Deref,
};

/// This enum represents all possible types of the input media
/// # Documentation
/// <https://core.telegram.org/bots/api#inputmedia>
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InputMediaType {
    Animation,
    Audio,
    Document,
    Photo,
    Video,
}

impl InputMediaType {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            InputMediaType::Animation => "animation",
            InputMediaType::Audio => "audio",
            InputMediaType::Document => "document",
            InputMediaType::Photo => "photo",
            InputMediaType::Video => "video",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [InputMediaType; 5] {
        &[
            InputMediaType::Animation,
            InputMediaType::Audio,
            InputMediaType::Document,
            InputMediaType::Photo,
            InputMediaType::Video,
        ]
    }
}

impl Deref for InputMediaType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Display for InputMediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<InputMediaType> for Box<str> {
    fn from(media_type: InputMediaType) -> Self {
        media_type.into()
    }
}

impl From<InputMediaType> for String {
    fn from(media_type: InputMediaType) -> Self {
        media_type.as_str().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for InputMediaType {
    fn eq(&self, other: &&'a str) -> bool {
        self == other
    }
}
