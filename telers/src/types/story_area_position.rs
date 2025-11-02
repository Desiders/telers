use serde::Serialize;

/// Describes the position of a clickable area within a story.
/// # Documentation
/// <https://core.telegram.org/bots/api#storyareaposition>
#[derive(Debug, Clone, PartialEq, Serialize)]
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
    pub fn new(
        x_percentage: f64,
        y_percentage: f64,
        width_percentage: f64,
        height_percentage: f64,
        rotation_angle: f64,
        corner_radius_percentage: f64,
    ) -> Self {
        Self {
            x_percentage,
            y_percentage,
            width_percentage,
            height_percentage,
            rotation_angle,
            corner_radius_percentage,
        }
    }

    pub fn x_percentage(self, val: f64) -> Self {
        Self {
            x_percentage: val,
            ..self
        }
    }

    pub fn y_percentage(self, val: f64) -> Self {
        Self {
            y_percentage: val,
            ..self
        }
    }

    pub fn width_percentage(self, val: f64) -> Self {
        Self {
            width_percentage: val,
            ..self
        }
    }

    pub fn height_percentage(self, val: f64) -> Self {
        Self {
            height_percentage: val,
            ..self
        }
    }

    pub fn rotation_angle(self, val: f64) -> Self {
        Self {
            rotation_angle: val,
            ..self
        }
    }

    pub fn corner_radius_percentage(self, val: f64) -> Self {
        Self {
            corner_radius_percentage: val,
            ..self
        }
    }
}
