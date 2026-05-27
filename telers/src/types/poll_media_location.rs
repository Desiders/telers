use serde::{Deserialize, Serialize};
/// Media is a shared location, information about the location
/// # Notes
/// This object represents a poll media from original field `location`.
/// # Documentation
/// <https://core.telegram.org/bots/api#pollmedia>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PollMediaLocation {
    /// Media is a shared location, information about the location
    pub location: crate::types::Location,
}
impl PollMediaLocation {
    /// Creates a new `PollMediaLocation`.
    ///
    /// # Arguments
    /// * `location` - Media is a shared location, information about the location
    #[must_use]
    pub fn new<T0: Into<crate::types::Location>>(location: T0) -> Self {
        Self {
            location: location.into(),
        }
    }

    /// Media is a shared location, information about the location
    #[must_use]
    pub fn location<T: Into<crate::types::Location>>(mut self, val: T) -> Self {
        self.location = val.into();
        self
    }
}
