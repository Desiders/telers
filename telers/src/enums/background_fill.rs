use crate::types::BackgroundFill;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object describes the way a background is filled based on the selected colors. Currently, it can be one of
/// - [`BackgroundFillSolid`]
/// - [`BackgroundFillGradient`]
/// - [`BackgroundFillFreeformGradient`]
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundfill>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum BackgroundFillType {
    #[strum(serialize = "solid")]
    Solid,
    #[strum(serialize = "gradient")]
    Gradient,
    #[strum(serialize = "freeform_gradient")]
    FreeformGradient,
}
impl BackgroundFillType {
    #[must_use]
    pub const fn all() -> [BackgroundFillType; 3usize] {
        [
            BackgroundFillType::Solid,
            BackgroundFillType::Gradient,
            BackgroundFillType::FreeformGradient,
        ]
    }
}
impl From<BackgroundFillType> for Box<str> {
    fn from(val: BackgroundFillType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<BackgroundFillType> for String {
    fn from(val: BackgroundFillType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for BackgroundFillType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a BackgroundFill> for BackgroundFillType {
    fn from(val: &'a BackgroundFill) -> Self {
        match val {
            BackgroundFill::Solid(_) => BackgroundFillType::Solid,
            BackgroundFill::Gradient(_) => BackgroundFillType::Gradient,
            BackgroundFill::FreeformGradient(_) => BackgroundFillType::FreeformGradient,
        }
    }
}
impl From<BackgroundFill> for BackgroundFillType {
    fn from(val: BackgroundFill) -> Self {
        BackgroundFillType::from(&val)
    }
}
