use serde::{Deserialize, Serialize};
/// This object describes the position on faces where a mask should be placed by default.
/// # Documentation
/// <https://core.telegram.org/bots/api#maskposition>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaskPosition {
    /// The part of the face relative to which the mask should be placed. One of `forehead`, `eyes`, `mouth`, or `chin`.
    pub point: Box<str>,
    /// Shift by X-axis measured in widths of the mask scaled to the face size, from left to right. For example, choosing -1.0 will place mask just to the left of the default mask position.
    pub x_shift: f64,
    /// Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom. For example, 1.0 will place the mask just below the default mask position.
    pub y_shift: f64,
    /// Mask scaling coefficient. For example, 2.0 means double size.
    pub scale: f64,
}
impl MaskPosition {
    /// Creates a new `MaskPosition`.
    ///
    /// # Arguments
    /// * `point` - The part of the face relative to which the mask should be placed. One of `forehead`, `eyes`, `mouth`, or `chin`.
    /// * `x_shift` - Shift by X-axis measured in widths of the mask scaled to the face size, from left to right. For example, choosing -1.0 will place mask just to the left of the default mask position.
    /// * `y_shift` - Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom. For example, 1.0 will place the mask just below the default mask position.
    /// * `scale` - Mask scaling coefficient. For example, 2.0 means double size.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<f64>, T2: Into<f64>, T3: Into<f64>>(
        point: T0,
        x_shift: T1,
        y_shift: T2,
        scale: T3,
    ) -> Self {
        Self {
            point: point.into(),
            x_shift: x_shift.into(),
            y_shift: y_shift.into(),
            scale: scale.into(),
        }
    }

    /// The part of the face relative to which the mask should be placed. One of `forehead`, `eyes`, `mouth`, or `chin`.
    #[must_use]
    pub fn point<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.point = val.into();
        self
    }

    /// Shift by X-axis measured in widths of the mask scaled to the face size, from left to right. For example, choosing -1.0 will place mask just to the left of the default mask position.
    #[must_use]
    pub fn x_shift<T: Into<f64>>(mut self, val: T) -> Self {
        self.x_shift = val.into();
        self
    }

    /// Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom. For example, 1.0 will place the mask just below the default mask position.
    #[must_use]
    pub fn y_shift<T: Into<f64>>(mut self, val: T) -> Self {
        self.y_shift = val.into();
        self
    }

    /// Mask scaling coefficient. For example, 2.0 means double size.
    #[must_use]
    pub fn scale<T: Into<f64>>(mut self, val: T) -> Self {
        self.scale = val.into();
        self
    }
}
