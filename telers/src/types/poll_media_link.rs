use serde::{Deserialize, Serialize};
/// The HTTP link attached to the poll option
/// # Notes
/// This object represents a poll media from original field `link`.
/// # Documentation
/// <https://core.telegram.org/bots/api#pollmedia>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PollMediaLink {
    /// The HTTP link attached to the poll option
    pub link: crate::types::Link,
}
impl PollMediaLink {
    /// Creates a new `PollMediaLink`.
    ///
    /// # Arguments
    /// * `link` - The HTTP link attached to the poll option
    #[must_use]
    pub fn new<T0: Into<crate::types::Link>>(link: T0) -> Self {
        Self {
            link: link.into(),
        }
    }

    /// The HTTP link attached to the poll option
    #[must_use]
    pub fn link<T: Into<crate::types::Link>>(mut self, val: T) -> Self {
        self.link = val.into();
        self
    }
}
