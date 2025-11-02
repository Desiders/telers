use serde::Serialize;

/// Describes a story area pointing to a unique gift. Currently, a story can have at most 1 unique gift area.
/// # Documentation
/// <https://core.telegram.org/bots/api#storyareatypeuniquegift>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StoryAreaTypeUniqueGift {
    /// Unique name of the gift
    pub name: String,
}

impl StoryAreaTypeUniqueGift {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
