use serde::{Deserialize, Serialize};
/// The background is a wallpaper in the JPEG format.
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundtypewallpaper>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackgroundTypeWallpaper {
    /// Document with the wallpaper
    pub document: Box<crate::types::Document>,
    /// Dimming of the background in dark themes, as a percentage; 0-100
    pub dark_theme_dimming: u8,
    /// `true`, if the wallpaper is downscaled to fit in a 450x450 square and then box-blurred with radius 12
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_blurred: Option<bool>,
    /// `true`, if the background moves slightly when the device is tilted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_moving: Option<bool>,
}
impl BackgroundTypeWallpaper {
    /// Creates a new `BackgroundTypeWallpaper`.
    ///
    /// # Arguments
    /// * `document` - Document with the wallpaper
    /// * `dark_theme_dimming` - Dimming of the background in dark themes, as a percentage; 0-100
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::Document>, T1: Into<u8>>(
        document: T0,
        dark_theme_dimming: T1,
    ) -> Self {
        Self {
            document: Box::new(document.into()),
            dark_theme_dimming: dark_theme_dimming.into(),
            is_blurred: None,
            is_moving: None,
        }
    }

    /// Document with the wallpaper
    #[must_use]
    pub fn document<T: Into<crate::types::Document>>(mut self, val: T) -> Self {
        self.document = Box::new(val.into());
        self
    }

    /// Dimming of the background in dark themes, as a percentage; 0-100
    #[must_use]
    pub fn dark_theme_dimming<T: Into<u8>>(mut self, val: T) -> Self {
        self.dark_theme_dimming = val.into();
        self
    }

    /// `true`, if the wallpaper is downscaled to fit in a 450x450 square and then box-blurred with radius 12
    #[must_use]
    pub fn is_blurred<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_blurred = Some(val.into());
        self
    }

    /// `true`, if the wallpaper is downscaled to fit in a 450x450 square and then box-blurred with radius 12
    #[must_use]
    pub fn is_blurred_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_blurred = val.map(Into::into);
        self
    }

    /// `true`, if the background moves slightly when the device is tilted
    #[must_use]
    pub fn is_moving<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_moving = Some(val.into());
        self
    }

    /// `true`, if the background moves slightly when the device is tilted
    #[must_use]
    pub fn is_moving_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_moving = val.map(Into::into);
        self
    }
}
