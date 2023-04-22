use std::fmt::{self, Debug};

/// This enum represents all possible types of the topic icon color
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum TopicIconColor {
    Blue,
    Yellow,
    Violet,
    Green,
    Rose,
    Red,
}

impl Debug for TopicIconColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TopicIconColor {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            TopicIconColor::Blue => "0x6FB9F0",
            TopicIconColor::Yellow => "0xFFD67E",
            TopicIconColor::Violet => "0xCB86DB",
            TopicIconColor::Green => "0x8EEE98",
            TopicIconColor::Rose => "0xFF93B2",
            TopicIconColor::Red => "0xFB6F5F",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [TopicIconColor; 6] {
        &[
            TopicIconColor::Blue,
            TopicIconColor::Yellow,
            TopicIconColor::Violet,
            TopicIconColor::Green,
            TopicIconColor::Rose,
            TopicIconColor::Red,
        ]
    }
}

impl<'a> PartialEq<&'a str> for TopicIconColor {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str() == *other
    }
}

impl From<TopicIconColor> for String {
    fn from(color: TopicIconColor) -> Self {
        color.as_str().to_string()
    }
}
