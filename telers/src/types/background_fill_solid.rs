use serde::{Deserialize, Serialize};
/// The background is filled using the selected color.
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundfillsolid>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackgroundFillSolid {
    /// The color of the background fill in the RGB24 format
    pub color: i32,
}
impl BackgroundFillSolid {
    /// Creates a new `BackgroundFillSolid`.
    ///
    /// # Arguments
    /// * `color` - The color of the background fill in the RGB24 format
    #[must_use]
    pub fn new<T0: Into<i32>>(color: T0) -> Self {
        Self {
            color: color.into(),
        }
    }

    /// The color of the background fill in the RGB24 format
    #[must_use]
    pub fn color<T: Into<i32>>(mut self, val: T) -> Self {
        self.color = val.into();
        self
    }
}
