use serde::{Deserialize, Serialize};
/// This object describes the type of a background. Currently, it can be one of
/// - [`crate::types::BackgroundTypeFill`]
/// - [`crate::types::BackgroundTypeWallpaper`]
/// - [`crate::types::BackgroundTypePattern`]
/// - [`crate::types::BackgroundTypeChatTheme`]
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundtype>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BackgroundType {
    Fill(crate::types::BackgroundTypeFill),
    Wallpaper(crate::types::BackgroundTypeWallpaper),
    Pattern(crate::types::BackgroundTypePattern),
    ChatTheme(crate::types::BackgroundTypeChatTheme),
}
impl BackgroundType {
    /// Helper method for field `dark_theme_dimming`.
    ///
    /// Dimming of the background in dark themes, as a percentage; 0-100
    #[must_use]
    pub fn dark_theme_dimming(&self) -> Option<u8> {
        match self {
            Self::Fill(val) => Some(val.dark_theme_dimming),
            Self::Wallpaper(val) => Some(val.dark_theme_dimming),
            _ => None,
        }
    }

    /// Helper method for field `document`.
    ///
    /// # Variants
    /// - `BackgroundTypeWallpaper`. Document with the wallpaper
    /// - `BackgroundTypePattern`. Document with the pattern
    #[must_use]
    pub fn document(&self) -> Option<&crate::types::Document> {
        match self {
            Self::Wallpaper(val) => Some(val.document.as_ref()),
            Self::Pattern(val) => Some(val.document.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `fill`.
    ///
    /// # Variants
    /// - `BackgroundTypeFill`. The background fill
    /// - `BackgroundTypePattern`. The background fill that is combined with the pattern
    #[must_use]
    pub fn fill(&self) -> Option<&crate::types::BackgroundFill> {
        match self {
            Self::Fill(val) => Some(&val.fill),
            Self::Pattern(val) => Some(&val.fill),
            _ => None,
        }
    }

    /// Helper method for field `intensity`.
    ///
    /// Intensity of the pattern when it is shown above the filled background; 0-100
    #[must_use]
    pub fn intensity(&self) -> Option<u8> {
        match self {
            Self::Pattern(val) => Some(val.intensity),
            _ => None,
        }
    }

    /// Helper method for field `is_blurred`.
    ///
    /// `true`, if the wallpaper is downscaled to fit in a 450x450 square and then box-blurred with radius 12
    #[must_use]
    pub fn is_blurred(&self) -> Option<bool> {
        match self {
            Self::Wallpaper(val) => val.is_blurred,
            _ => None,
        }
    }

    /// Helper method for field `is_inverted`.
    ///
    /// `true`, if the background fill must be applied only to the pattern itself. All other pixels are black in this case. For dark themes only.
    #[must_use]
    pub fn is_inverted(&self) -> Option<bool> {
        match self {
            Self::Pattern(val) => val.is_inverted,
            _ => None,
        }
    }

    /// Helper method for field `is_moving`.
    ///
    /// `true`, if the background moves slightly when the device is tilted
    #[must_use]
    pub fn is_moving(&self) -> Option<bool> {
        match self {
            Self::Wallpaper(val) => val.is_moving,
            Self::Pattern(val) => val.is_moving,
            _ => None,
        }
    }

    /// Helper method for field `theme_name`.
    ///
    /// Name of the chat theme, which is usually an emoji
    #[must_use]
    pub fn theme_name(&self) -> Option<&str> {
        match self {
            Self::ChatTheme(val) => Some(val.theme_name.as_ref()),
            _ => None,
        }
    }

    /// Helper method for nested field `bottom_color`.
    #[must_use]
    pub fn bottom_color(&self) -> Option<i32> {
        match self {
            Self::Fill(val) => {
                let inner = &val.fill;
                crate::types::BackgroundFill::bottom_color(inner)
            }
            Self::Pattern(val) => {
                let inner = &val.fill;
                crate::types::BackgroundFill::bottom_color(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `color`.
    #[must_use]
    pub fn color(&self) -> Option<i32> {
        match self {
            Self::Fill(val) => {
                let inner = &val.fill;
                crate::types::BackgroundFill::color(inner)
            }
            Self::Pattern(val) => {
                let inner = &val.fill;
                crate::types::BackgroundFill::color(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `colors`.
    #[must_use]
    pub fn colors(&self) -> Option<&[i32]> {
        match self {
            Self::Fill(val) => {
                let inner = &val.fill;
                crate::types::BackgroundFill::colors(inner)
            }
            Self::Pattern(val) => {
                let inner = &val.fill;
                crate::types::BackgroundFill::colors(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_id`.
    #[must_use]
    pub fn file_id(&self) -> Option<&str> {
        match self {
            Self::Wallpaper(val) => {
                let inner = val.document.as_ref();
                Some(inner.file_id.as_ref())
            }
            Self::Pattern(val) => {
                let inner = val.document.as_ref();
                Some(inner.file_id.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_name`.
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        match self {
            Self::Wallpaper(val) => {
                let inner = val.document.as_ref();
                inner.file_name.as_deref()
            }
            Self::Pattern(val) => {
                let inner = val.document.as_ref();
                inner.file_name.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_size`.
    #[must_use]
    pub fn file_size(&self) -> Option<i64> {
        match self {
            Self::Wallpaper(val) => {
                let inner = val.document.as_ref();
                inner.file_size
            }
            Self::Pattern(val) => {
                let inner = val.document.as_ref();
                inner.file_size
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_unique_id`.
    #[must_use]
    pub fn file_unique_id(&self) -> Option<&str> {
        match self {
            Self::Wallpaper(val) => {
                let inner = val.document.as_ref();
                Some(inner.file_unique_id.as_ref())
            }
            Self::Pattern(val) => {
                let inner = val.document.as_ref();
                Some(inner.file_unique_id.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `mime_type`.
    #[must_use]
    pub fn mime_type(&self) -> Option<&str> {
        match self {
            Self::Wallpaper(val) => {
                let inner = val.document.as_ref();
                inner.mime_type.as_deref()
            }
            Self::Pattern(val) => {
                let inner = val.document.as_ref();
                inner.mime_type.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `rotation_angle`.
    #[must_use]
    pub fn rotation_angle(&self) -> Option<u16> {
        match self {
            Self::Fill(val) => {
                let inner = &val.fill;
                crate::types::BackgroundFill::rotation_angle(inner)
            }
            Self::Pattern(val) => {
                let inner = &val.fill;
                crate::types::BackgroundFill::rotation_angle(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `thumbnail`.
    #[must_use]
    pub fn thumbnail(&self) -> Option<&crate::types::PhotoSize> {
        match self {
            Self::Wallpaper(val) => {
                let inner = val.document.as_ref();
                inner.thumbnail.as_ref()
            }
            Self::Pattern(val) => {
                let inner = val.document.as_ref();
                inner.thumbnail.as_ref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `top_color`.
    #[must_use]
    pub fn top_color(&self) -> Option<i32> {
        match self {
            Self::Fill(val) => {
                let inner = &val.fill;
                crate::types::BackgroundFill::top_color(inner)
            }
            Self::Pattern(val) => {
                let inner = &val.fill;
                crate::types::BackgroundFill::top_color(inner)
            }
            _ => None,
        }
    }
}
impl From<crate::types::BackgroundTypeFill> for BackgroundType {
    fn from(val: crate::types::BackgroundTypeFill) -> Self {
        Self::Fill(val)
    }
}
impl TryFrom<BackgroundType> for crate::types::BackgroundTypeFill {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: BackgroundType) -> Result<Self, Self::Error> {
        if let BackgroundType::Fill(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(BackgroundType),
                stringify!(BackgroundTypeFill),
            ))
        }
    }
}
impl From<crate::types::BackgroundTypeWallpaper> for BackgroundType {
    fn from(val: crate::types::BackgroundTypeWallpaper) -> Self {
        Self::Wallpaper(val)
    }
}
impl TryFrom<BackgroundType> for crate::types::BackgroundTypeWallpaper {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: BackgroundType) -> Result<Self, Self::Error> {
        if let BackgroundType::Wallpaper(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(BackgroundType),
                stringify!(BackgroundTypeWallpaper),
            ))
        }
    }
}
impl From<crate::types::BackgroundTypePattern> for BackgroundType {
    fn from(val: crate::types::BackgroundTypePattern) -> Self {
        Self::Pattern(val)
    }
}
impl TryFrom<BackgroundType> for crate::types::BackgroundTypePattern {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: BackgroundType) -> Result<Self, Self::Error> {
        if let BackgroundType::Pattern(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(BackgroundType),
                stringify!(BackgroundTypePattern),
            ))
        }
    }
}
impl From<crate::types::BackgroundTypeChatTheme> for BackgroundType {
    fn from(val: crate::types::BackgroundTypeChatTheme) -> Self {
        Self::ChatTheme(val)
    }
}
impl TryFrom<BackgroundType> for crate::types::BackgroundTypeChatTheme {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: BackgroundType) -> Result<Self, Self::Error> {
        if let BackgroundType::ChatTheme(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(BackgroundType),
                stringify!(BackgroundTypeChatTheme),
            ))
        }
    }
}
