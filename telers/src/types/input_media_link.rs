use serde::{Deserialize, Serialize};
/// Represents an HTTP link to be sent.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputmedialink>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputMediaLink {
    /// HTTP URL of the link
    pub url: Box<str>,
}
impl InputMediaLink {
    /// Creates a new `InputMediaLink`.
    ///
    /// # Arguments
    /// * `url` - HTTP URL of the link
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(url: T0) -> Self {
        Self {
            url: url.into(),
        }
    }

    /// HTTP URL of the link
    #[must_use]
    pub fn url<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.url = val.into();
        self
    }
}
