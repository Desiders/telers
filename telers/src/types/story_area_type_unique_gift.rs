use serde::{Deserialize, Serialize};
/// Describes a story area pointing to a unique gift. Currently, a story can have at most 1 unique gift area.
/// # Documentation
/// <https://core.telegram.org/bots/api#storyareatypeuniquegift>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoryAreaTypeUniqueGift {
    /// Unique name of the gift
    pub name: Box<str>,
}
impl StoryAreaTypeUniqueGift {
    /// Creates a new `StoryAreaTypeUniqueGift`.
    ///
    /// # Arguments
    /// * `name` - Unique name of the gift
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(name: T0) -> Self {
        Self { name: name.into() }
    }

    /// Unique name of the gift
    #[must_use]
    pub fn name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.name = val.into();
        self
    }
}
