use serde::{Deserialize, Serialize};
/// The background is automatically filled based on the selected colors.
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundtypefill>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackgroundTypeFill {
    /// The background fill
    pub fill: crate::types::BackgroundFill,
    /// Dimming of the background in dark themes, as a percentage; 0-100
    pub dark_theme_dimming: u8,
}
impl BackgroundTypeFill {
    /// Creates a new `BackgroundTypeFill`.
    ///
    /// # Arguments
    /// * `fill` - The background fill
    /// * `dark_theme_dimming` - Dimming of the background in dark themes, as a percentage; 0-100
    #[must_use]
    pub fn new<T0: Into<crate::types::BackgroundFill>, T1: Into<u8>>(
        fill: T0,
        dark_theme_dimming: T1,
    ) -> Self {
        Self {
            fill: fill.into(),
            dark_theme_dimming: dark_theme_dimming.into(),
        }
    }

    /// The background fill
    #[must_use]
    pub fn fill<T: Into<crate::types::BackgroundFill>>(self, val: T) -> Self {
        let mut this = self;
        this.fill = val.into();
        this
    }

    /// Dimming of the background in dark themes, as a percentage; 0-100
    #[must_use]
    pub fn dark_theme_dimming<T: Into<u8>>(self, val: T) -> Self {
        let mut this = self;
        this.dark_theme_dimming = val.into();
        this
    }
}
