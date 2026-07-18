use serde::{Deserialize, Serialize};
/// This object describes the way a background is filled based on the selected colors. Currently, it can be one of
/// - [`crate::types::BackgroundFillSolid`]
/// - [`crate::types::BackgroundFillGradient`]
/// - [`crate::types::BackgroundFillFreeformGradient`]
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundfill>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BackgroundFill {
    Solid(crate::types::BackgroundFillSolid),
    Gradient(crate::types::BackgroundFillGradient),
    FreeformGradient(crate::types::BackgroundFillFreeformGradient),
    /// Content unknown to this version of the library
    #[serde(untagged)]
    Unknown(crate::types::BackgroundFillUnknown),
}
impl BackgroundFill {
    /// Helper method for field `bottom_color`.
    ///
    /// Bottom color of the gradient in the RGB24 format
    #[must_use]
    pub fn bottom_color(&self) -> Option<i32> {
        match self {
            Self::Gradient(val) => Some(val.bottom_color),
            _ => None,
        }
    }

    /// Helper method for field `color`.
    ///
    /// The color of the background fill in the RGB24 format
    #[must_use]
    pub fn color(&self) -> Option<i32> {
        match self {
            Self::Solid(val) => Some(val.color),
            _ => None,
        }
    }

    /// Helper method for field `colors`.
    ///
    /// A list of the 3 or 4 base colors that are used to generate the freeform gradient in the RGB24 format
    #[must_use]
    pub fn colors(&self) -> Option<&[i32]> {
        match self {
            Self::FreeformGradient(val) => Some(val.colors.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `rotation_angle`.
    ///
    /// Clockwise rotation angle of the background fill in degrees; 0-359
    #[must_use]
    pub fn rotation_angle(&self) -> Option<u16> {
        match self {
            Self::Gradient(val) => Some(val.rotation_angle),
            _ => None,
        }
    }

    /// Helper method for field `top_color`.
    ///
    /// Top color of the gradient in the RGB24 format
    #[must_use]
    pub fn top_color(&self) -> Option<i32> {
        match self {
            Self::Gradient(val) => Some(val.top_color),
            _ => None,
        }
    }
}
impl From<crate::types::BackgroundFillSolid> for BackgroundFill {
    fn from(val: crate::types::BackgroundFillSolid) -> Self {
        Self::Solid(val)
    }
}
impl TryFrom<BackgroundFill> for crate::types::BackgroundFillSolid {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: BackgroundFill) -> Result<Self, Self::Error> {
        if let BackgroundFill::Solid(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(BackgroundFill),
                stringify!(BackgroundFillSolid),
            ))
        }
    }
}
impl From<crate::types::BackgroundFillGradient> for BackgroundFill {
    fn from(val: crate::types::BackgroundFillGradient) -> Self {
        Self::Gradient(val)
    }
}
impl TryFrom<BackgroundFill> for crate::types::BackgroundFillGradient {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: BackgroundFill) -> Result<Self, Self::Error> {
        if let BackgroundFill::Gradient(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(BackgroundFill),
                stringify!(BackgroundFillGradient),
            ))
        }
    }
}
impl From<crate::types::BackgroundFillFreeformGradient> for BackgroundFill {
    fn from(val: crate::types::BackgroundFillFreeformGradient) -> Self {
        Self::FreeformGradient(val)
    }
}
impl TryFrom<BackgroundFill> for crate::types::BackgroundFillFreeformGradient {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: BackgroundFill) -> Result<Self, Self::Error> {
        if let BackgroundFill::FreeformGradient(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(BackgroundFill),
                stringify!(BackgroundFillFreeformGradient),
            ))
        }
    }
}
impl From<crate::types::BackgroundFillUnknown> for BackgroundFill {
    fn from(val: crate::types::BackgroundFillUnknown) -> Self {
        Self::Unknown(val)
    }
}
impl TryFrom<BackgroundFill> for crate::types::BackgroundFillUnknown {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: BackgroundFill) -> Result<Self, Self::Error> {
        if let BackgroundFill::Unknown(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(BackgroundFill),
                stringify!(BackgroundFillUnknown),
            ))
        }
    }
}
