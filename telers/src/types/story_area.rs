use serde::{Deserialize, Serialize};
/// Describes a clickable area on a story media.
/// # Documentation
/// <https://core.telegram.org/bots/api#storyarea>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoryArea {
    /// Position of the area
    pub position: crate::types::StoryAreaPosition,
    /// Type of the area
    pub r#type: crate::types::StoryAreaType,
}
impl StoryArea {
    /// Creates a new `StoryArea`.
    ///
    /// # Arguments
    /// * `position` - Position of the area
    /// * `type` - Type of the area
    #[must_use]
    pub fn new<T0: Into<crate::types::StoryAreaPosition>, T1: Into<crate::types::StoryAreaType>>(
        position: T0,
        r#type: T1,
    ) -> Self {
        Self {
            position: position.into(),
            r#type: r#type.into(),
        }
    }

    /// Position of the area
    #[must_use]
    pub fn position<T: Into<crate::types::StoryAreaPosition>>(mut self, val: T) -> Self {
        self.position = val.into();
        self
    }

    /// Type of the area
    #[must_use]
    pub fn r#type<T: Into<crate::types::StoryAreaType>>(mut self, val: T) -> Self {
        self.r#type = val.into();
        self
    }
}
