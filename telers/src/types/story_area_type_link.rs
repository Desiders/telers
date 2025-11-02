use serde::Serialize;

/// Describes a story area pointing to an HTTP or tg:// link. Currently, a story can have up to 3 link areas.
/// # Documentation
/// <https://core.telegram.org/bots/api#storyareatypelink>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct StoryAreaTypeLink {
    /// HTTP or tg:// URL to be opened when the area is clicked
    pub url: String,
}

impl StoryAreaTypeLink {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    pub fn url(self, val: impl Into<String>) -> Self {
        Self {
            url: val.into(),
            ..self
        }
    }
}
