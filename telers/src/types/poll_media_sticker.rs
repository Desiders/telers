use serde::{Deserialize, Serialize};
/// Media is a sticker, information about the sticker; currently, for poll options only
/// # Notes
/// This object represents a poll media from original field `sticker`.
/// # Documentation
/// <https://core.telegram.org/bots/api#pollmedia>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PollMediaSticker {
    /// Media is a sticker, information about the sticker; currently, for poll options only
    pub sticker: Box<crate::types::Sticker>,
}
impl PollMediaSticker {
    /// Creates a new `PollMediaSticker`.
    ///
    /// # Arguments
    /// * `sticker` - Media is a sticker, information about the sticker; currently, for poll options only
    #[must_use]
    pub fn new<T0: Into<crate::types::Sticker>>(sticker: T0) -> Self {
        Self {
            sticker: Box::new(sticker.into()),
        }
    }

    /// Media is a sticker, information about the sticker; currently, for poll options only
    #[must_use]
    pub fn sticker<T: Into<crate::types::Sticker>>(mut self, val: T) -> Self {
        self.sticker = Box::new(val.into());
        self
    }
}
