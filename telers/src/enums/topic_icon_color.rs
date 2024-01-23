use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// This enum represents all possible types of the topic icon color
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum TopicIconColor {
    #[strum(serialize = "0x6FB9F0")]
    Blue,
    #[strum(serialize = "0xFFD67E")]
    Yellow,
    #[strum(serialize = "0xCB86DB")]
    Violet,
    #[strum(serialize = "0x8EEE98")]
    Green,
    #[strum(serialize = "0xFF93B2")]
    Rose,
    #[strum(serialize = "0xFB6F5F")]
    Red,
}

impl TopicIconColor {
    #[must_use]
    pub const fn all() -> [TopicIconColor; 6] {
        [
            TopicIconColor::Blue,
            TopicIconColor::Yellow,
            TopicIconColor::Violet,
            TopicIconColor::Green,
            TopicIconColor::Rose,
            TopicIconColor::Red,
        ]
    }
}

impl From<TopicIconColor> for Box<str> {
    fn from(color: TopicIconColor) -> Self {
        Into::<&'static str>::into(color).into()
    }
}

impl From<TopicIconColor> for String {
    fn from(color: TopicIconColor) -> Self {
        color.as_ref().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for TopicIconColor {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
