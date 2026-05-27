use serde::{Deserialize, Serialize};
/// Media is a general file, information about the file; currently, can't be received in a poll option
/// # Notes
/// This object represents a poll media from original field `document`.
/// # Documentation
/// <https://core.telegram.org/bots/api#pollmedia>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PollMediaDocument {
    /// Media is a general file, information about the file; currently, can't be received in a poll option
    pub document: Box<crate::types::Document>,
}
impl PollMediaDocument {
    /// Creates a new `PollMediaDocument`.
    ///
    /// # Arguments
    /// * `document` - Media is a general file, information about the file; currently, can't be received in a poll option
    #[must_use]
    pub fn new<T0: Into<crate::types::Document>>(document: T0) -> Self {
        Self {
            document: Box::new(document.into()),
        }
    }

    /// Media is a general file, information about the file; currently, can't be received in a poll option
    #[must_use]
    pub fn document<T: Into<crate::types::Document>>(mut self, val: T) -> Self {
        self.document = Box::new(val.into());
        self
    }
}
