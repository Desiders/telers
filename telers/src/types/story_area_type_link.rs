/// Describes a story area pointing to an HTTP or `tg`:// link. Currently, a story can have up to 3 link areas.
/// # Documentation
/// <https://core.telegram.org/bots/api#storyareatypelink>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct StoryAreaTypeLink {
    /// HTTP or `tg`:// URL to be opened when the area is clicked
    pub url: Box<str>,
}
impl StoryAreaTypeLink {
    /// Creates a new `StoryAreaTypeLink`.
    ///
    /// # Arguments
    /// * `url` - HTTP or `tg`:// URL to be opened when the area is clicked
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(url: T0) -> Self {
        Self {
            url: url.into(),
        }
    }

    /// HTTP or `tg`:// URL to be opened when the area is clicked
    #[must_use]
    pub fn url<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.url = val.into();
        self
    }
}
