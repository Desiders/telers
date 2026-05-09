use serde::{Deserialize, Serialize};
/// The background is a gradient fill.
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundfillgradient>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackgroundFillGradient {
    /// Top color of the gradient in the RGB24 format
    pub top_color: i64,
    /// Bottom color of the gradient in the RGB24 format
    pub bottom_color: i64,
    /// Clockwise rotation angle of the background fill in degrees; 0-359
    pub rotation_angle: u16,
}
impl BackgroundFillGradient {
    /// Creates a new `BackgroundFillGradient`.
    ///
    /// # Arguments
    /// * `top_color` - Top color of the gradient in the RGB24 format
    /// * `bottom_color` - Bottom color of the gradient in the RGB24 format
    /// * `rotation_angle` - Clockwise rotation angle of the background fill in degrees; 0-359
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>, T2: Into<u16>>(
        top_color: T0,
        bottom_color: T1,
        rotation_angle: T2,
    ) -> Self {
        Self {
            top_color: top_color.into(),
            bottom_color: bottom_color.into(),
            rotation_angle: rotation_angle.into(),
        }
    }

    /// Top color of the gradient in the RGB24 format
    #[must_use]
    pub fn top_color<T: Into<i64>>(mut self, val: T) -> Self {
        self.top_color = val.into();
        self
    }

    /// Bottom color of the gradient in the RGB24 format
    #[must_use]
    pub fn bottom_color<T: Into<i64>>(mut self, val: T) -> Self {
        self.bottom_color = val.into();
        self
    }

    /// Clockwise rotation angle of the background fill in degrees; 0-359
    #[must_use]
    pub fn rotation_angle<T: Into<u16>>(mut self, val: T) -> Self {
        self.rotation_angle = val.into();
        self
    }
}
