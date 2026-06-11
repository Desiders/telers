use serde::{Deserialize, Serialize};
/// Represents an HTTP link.
/// # Documentation
/// <https://core.telegram.org/bots/api#link>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    /// URL of the link
    pub url: Box<str>,
}
impl Link {
    /// Creates a new `Link`.
    ///
    /// # Arguments
    /// * `url` - URL of the link
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(url: T0) -> Self {
        Self {
            url: url.into(),
        }
    }

    /// URL of the link
    #[must_use]
    pub fn url<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.url = val.into();
        self
    }
}
