use std::fmt::{self, Debug, Display};

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

impl Display for MaskPositionPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<MaskPositionPoint> for String {
    fn from(point: MaskPositionPoint) -> Self {
        point.as_str().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for MaskPositionPoint {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str() == *other
    }
}
