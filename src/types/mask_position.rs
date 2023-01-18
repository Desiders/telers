use serde::{Deserialize, Serialize};

/// This object describes the position on faces where a mask should be placed by default.
/// <https://core.telegram.org/bots/api#maskposition>
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MaskPosition {
    /// The part of the face relative to which the mask should be placed. One of 'forehead', 'eyes', 'mouth', or 'chin'.
    pub point: String,
    /// Shift by X-axis measured in widths of the mask scaled to the face size, from left to right. For example, choosing -1.0 will place mask just to the left of the default mask position.
    pub x_shift: f64,
    /// Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom. For example, 1.0 will place the mask just below the default mask position.
    pub y_shift: f64,
    /// Mask scaling coefficient. For example, 2.0 means double size.
    pub scale: f64,
}

impl MaskPosition {
    #[must_use]
    pub fn new<T: Into<String>>(point: T, x_shift: f64, y_shift: f64, scale: f64) -> Self {
        Self {
            point: point.into(),
            x_shift,
            y_shift,
            scale,
        }
    }

    #[must_use]
    pub fn point<T: Into<String>>(mut self, point: T) -> Self {
        self.point = point.into();
        self
    }

    #[must_use]
    pub fn x_shift(mut self, x_shift: f64) -> Self {
        self.x_shift = x_shift;
        self
    }

    #[must_use]
    pub fn y_shift(mut self, y_shift: f64) -> Self {
        self.y_shift = y_shift;
        self
    }

    #[must_use]
    pub fn scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }
}
