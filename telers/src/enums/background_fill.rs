use crate::types::BackgroundFill;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object describes the way a background is filled based on the selected colors. Currently, it can be one of
/// - [`crate::types::BackgroundFillSolid`]
/// - [`crate::types::BackgroundFillGradient`]
/// - [`crate::types::BackgroundFillFreeformGradient`]
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundfill>
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
pub enum BackgroundFillType {
    #[strum(serialize = "solid")]
    Solid,
    #[strum(serialize = "gradient")]
    Gradient,
    #[strum(serialize = "freeform_gradient")]
    FreeformGradient,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl BackgroundFillType {
    #[must_use]
    pub const fn all() -> [BackgroundFillType; 4usize] {
        [
            BackgroundFillType::Solid,
            BackgroundFillType::Gradient,
            BackgroundFillType::FreeformGradient,
            BackgroundFillType::Unknown,
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
            BackgroundFill::Unknown(_) => BackgroundFillType::Unknown,
        }
    }
}
impl From<BackgroundFill> for BackgroundFillType {
    fn from(val: BackgroundFill) -> Self {
        BackgroundFillType::from(&val)
    }
}
