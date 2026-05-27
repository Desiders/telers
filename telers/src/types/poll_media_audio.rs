use serde::{Deserialize, Serialize};
/// Media is an audio file, information about the file; currently, can't be received in a poll option
/// # Notes
/// This object represents a poll media from original field `audio`.
/// # Documentation
/// <https://core.telegram.org/bots/api#pollmedia>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PollMediaAudio {
    /// Media is an audio file, information about the file; currently, can't be received in a poll option
    pub audio: Box<crate::types::Audio>,
}
impl PollMediaAudio {
    /// Creates a new `PollMediaAudio`.
    ///
    /// # Arguments
    /// * `audio` - Media is an audio file, information about the file; currently, can't be received in a poll option
    #[must_use]
    pub fn new<T0: Into<crate::types::Audio>>(audio: T0) -> Self {
        Self {
            audio: Box::new(audio.into()),
        }
    }

    /// Media is an audio file, information about the file; currently, can't be received in a poll option
    #[must_use]
    pub fn audio<T: Into<crate::types::Audio>>(mut self, val: T) -> Self {
        self.audio = Box::new(val.into());
        self
    }
}
