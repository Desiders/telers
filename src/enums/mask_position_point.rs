use std::fmt::{self, Debug};

pub enum MaskPositionPoint {
    Forehead,
    Eyes,
    Mouth,
    Chin,
}

impl Debug for MaskPositionPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl MaskPositionPoint {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Forehead => "forehead",
            Self::Eyes => "eyes",
            Self::Mouth => "mouth",
            Self::Chin => "chin",
        }
    }

    pub fn all() -> &'static [Self; 4] {
        &[Self::Forehead, Self::Eyes, Self::Mouth, Self::Chin]
    }
}

impl From<MaskPositionPoint> for String {
    fn from(point: MaskPositionPoint) -> Self {
        point.as_str().to_string()
    }
}

impl<'a> From<&'a MaskPositionPoint> for String {
    fn from(point: &'a MaskPositionPoint) -> Self {
        point.as_str().to_string()
    }
}
