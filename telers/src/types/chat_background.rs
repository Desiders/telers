use serde::{Deserialize, Serialize};
/// This object represents a chat background.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatbackground>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatBackground {
    /// Type of the background
    pub r#type: Box<crate::types::BackgroundType>,
}
impl ChatBackground {
    /// Creates a new `ChatBackground`.
    ///
    /// # Arguments
    /// * `type` - Type of the background
    #[must_use]
    pub fn new<T0: Into<crate::types::BackgroundType>>(r#type: T0) -> Self {
        Self {
            r#type: Box::new(r#type.into()),
        }
    }

    /// Type of the background
    #[must_use]
    pub fn r#type<T: Into<crate::types::BackgroundType>>(mut self, val: T) -> Self {
        self.r#type = Box::new(val.into());
        self
    }
}
