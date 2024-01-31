use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// This enum represents all possible types of the mask position point
/// # Documentation
/// <https://core.telegram.org/bots/api#maskposition>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum MaskPositionPoint {
    #[strum(serialize = "forehead")]
    Forehead,
    #[strum(serialize = "eyes")]
    Eyes,
    #[strum(serialize = "mouth")]
    Mouth,
    #[strum(serialize = "chin")]
    Chin,
}

impl MaskPositionPoint {
    #[must_use]
    pub const fn all() -> [Self; 4] {
        [Self::Forehead, Self::Eyes, Self::Mouth, Self::Chin]
    }
}

impl From<MaskPositionPoint> for Box<str> {
    fn from(point: MaskPositionPoint) -> Self {
        Into::<&'static str>::into(point).into()
    }
}

impl From<MaskPositionPoint> for String {
    fn from(point: MaskPositionPoint) -> Self {
        point.as_ref().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for MaskPositionPoint {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
