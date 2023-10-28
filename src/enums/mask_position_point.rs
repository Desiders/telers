use std::{
    fmt::{self, Debug, Display},
    ops::Deref,
};

/// This enum represents all possible types of the mask position point
/// # Documentation
/// <https://core.telegram.org/bots/api#maskposition>
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MaskPositionPoint {
    Forehead,
    Eyes,
    Mouth,
    Chin,
}

impl MaskPositionPoint {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Forehead => "forehead",
            Self::Eyes => "eyes",
            Self::Mouth => "mouth",
            Self::Chin => "chin",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [Self; 4] {
        &[Self::Forehead, Self::Eyes, Self::Mouth, Self::Chin]
    }
}

impl Deref for MaskPositionPoint {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Display for MaskPositionPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<MaskPositionPoint> for Box<str> {
    fn from(point: MaskPositionPoint) -> Self {
        point.into()
    }
}

impl From<MaskPositionPoint> for String {
    fn from(point: MaskPositionPoint) -> Self {
        point.as_str().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for MaskPositionPoint {
    fn eq(&self, other: &&'a str) -> bool {
        self == other
    }
}
