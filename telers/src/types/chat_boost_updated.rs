use serde::{Deserialize, Serialize};
/// This object represents a boost added to a chat or changed.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboostupdated>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatBoostUpdated {
    /// Chat which was boosted
    pub chat: Box<crate::types::Chat>,
    /// Information about the chat boost
    pub boost: crate::types::ChatBoost,
}
impl ChatBoostUpdated {
    /// Creates a new `ChatBoostUpdated`.
    ///
    /// # Arguments
    /// * `chat` - Chat which was boosted
    /// * `boost` - Information about the chat boost
    #[must_use]
    pub fn new<T0: Into<crate::types::Chat>, T1: Into<crate::types::ChatBoost>>(
        chat: T0,
        boost: T1,
    ) -> Self {
        Self {
            chat: Box::new(chat.into()),
            boost: boost.into(),
        }
    }

    /// Chat which was boosted
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(mut self, val: T) -> Self {
        self.chat = Box::new(val.into());
        self
    }

    /// Information about the chat boost
    #[must_use]
    pub fn boost<T: Into<crate::types::ChatBoost>>(mut self, val: T) -> Self {
        self.boost = val.into();
        self
    }
}
