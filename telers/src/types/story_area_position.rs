use serde::{Deserialize, Serialize};
/// Describes the position of a clickable area within a story.
/// # Documentation
/// <https://core.telegram.org/bots/api#storyareaposition>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoryAreaPosition {
    /// The abscissa of the area's center, as a percentage of the media width
    pub x_percentage: f64,
    /// The ordinate of the area's center, as a percentage of the media height
    pub y_percentage: f64,
    /// The width of the area's rectangle, as a percentage of the media width
    pub width_percentage: f64,
    /// The height of the area's rectangle, as a percentage of the media height
    pub height_percentage: f64,
    /// The clockwise rotation angle of the rectangle, in degrees; 0-360
    pub rotation_angle: f64,
    /// The radius of the rectangle corner rounding, as a percentage of the media width
    pub corner_radius_percentage: f64,
}
impl StoryAreaPosition {
    /// Creates a new `StoryAreaPosition`.
    ///
    /// # Arguments
    /// * `x_percentage` - The abscissa of the area's center, as a percentage of the media width
    /// * `y_percentage` - The ordinate of the area's center, as a percentage of the media height
    /// * `width_percentage` - The width of the area's rectangle, as a percentage of the media width
    /// * `height_percentage` - The height of the area's rectangle, as a percentage of the media height
    /// * `rotation_angle` - The clockwise rotation angle of the rectangle, in degrees; 0-360
    /// * `corner_radius_percentage` - The radius of the rectangle corner rounding, as a percentage of the media width
    #[must_use]
    pub fn new<
        T0: Into<f64>,
        T1: Into<f64>,
        T2: Into<f64>,
        T3: Into<f64>,
        T4: Into<f64>,
        T5: Into<f64>,
    >(
        x_percentage: T0,
        y_percentage: T1,
        width_percentage: T2,
        height_percentage: T3,
        rotation_angle: T4,
        corner_radius_percentage: T5,
    ) -> Self {
        Self {
            x_percentage: x_percentage.into(),
            y_percentage: y_percentage.into(),
            width_percentage: width_percentage.into(),
            height_percentage: height_percentage.into(),
            rotation_angle: rotation_angle.into(),
            corner_radius_percentage: corner_radius_percentage.into(),
        }
    }

    /// The abscissa of the area's center, as a percentage of the media width
    #[must_use]
    pub fn x_percentage<T: Into<f64>>(self, val: T) -> Self {
        let mut this = self;
        this.x_percentage = val.into();
        this
    }

    /// The ordinate of the area's center, as a percentage of the media height
    #[must_use]
    pub fn y_percentage<T: Into<f64>>(self, val: T) -> Self {
        let mut this = self;
        this.y_percentage = val.into();
        this
    }

    /// The width of the area's rectangle, as a percentage of the media width
    #[must_use]
    pub fn width_percentage<T: Into<f64>>(self, val: T) -> Self {
        let mut this = self;
        this.width_percentage = val.into();
        this
    }

    /// The height of the area's rectangle, as a percentage of the media height
    #[must_use]
    pub fn height_percentage<T: Into<f64>>(self, val: T) -> Self {
        let mut this = self;
        this.height_percentage = val.into();
        this
    }

    /// The clockwise rotation angle of the rectangle, in degrees; 0-360
    #[must_use]
    pub fn rotation_angle<T: Into<f64>>(self, val: T) -> Self {
        let mut this = self;
        this.rotation_angle = val.into();
        this
    }

    /// The radius of the rectangle corner rounding, as a percentage of the media width
    #[must_use]
    pub fn corner_radius_percentage<T: Into<f64>>(self, val: T) -> Self {
        let mut this = self;
        this.corner_radius_percentage = val.into();
        this
    }
}
