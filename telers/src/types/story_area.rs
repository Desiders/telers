use serde::Serialize;

use super::{StoryAreaPosition, StoryAreaType};

/// Describes a clickable area on a story media.
/// # Documentation
/// <https://core.telegram.org/bots/api#storyarea>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StoryArea {
    /// Position of the area
    pub position: StoryAreaPosition,
    /// Type of the area
    #[serde(rename = "type")]
    pub area_type: StoryAreaType,
}

impl StoryArea {
    pub fn new(position: StoryAreaPosition, area_type: StoryAreaType) -> Self {
        Self {
            position,
            area_type,
        }
    }

    pub fn position(self, val: StoryAreaPosition) -> Self {
        Self {
            position: val,
            ..self
        }
    }

    pub fn area_type(self, val: StoryAreaType) -> Self {
        Self {
            area_type: val,
            ..self
        }
    }
}
